use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Hasher {
    fn new_internal(key: &CVWords, flags: u8) -> Self {
        Self {
            key: *key,
            chunk_state: ChunkState::new(key, 0, flags, Platform::detect()),
            initial_chunk_counter: 0,
            cv_stack: ArrayVec::new(),
        }
    }
    /// Construct a new `Hasher` for the regular hash function.
    pub fn new() -> Self {
        Self::new_internal(IV, 0)
    }
    /// Construct a new `Hasher` for the keyed hash function. See
    /// [`keyed_hash`].
    ///
    /// [`keyed_hash`]: fn.keyed_hash.html
    pub fn new_keyed(key: &[u8; KEY_LEN]) -> Self {
        let key_words = platform::words_from_le_bytes_32(key);
        Self::new_internal(&key_words, KEYED_HASH)
    }
    /// Construct a new `Hasher` for the key derivation function. See
    /// [`derive_key`]. The context string should be hardcoded, globally
    /// unique, and application-specific.
    ///
    /// [`derive_key`]: fn.derive_key.html
    pub fn new_derive_key(context: &str) -> Self {
        let context_key = hazmat::hash_derive_key_context(context);
        let context_key_words = platform::words_from_le_bytes_32(&context_key);
        Self::new_internal(&context_key_words, DERIVE_KEY_MATERIAL)
    }
    /// Reset the `Hasher` to its initial state.
    ///
    /// This is functionally the same as overwriting the `Hasher` with a new
    /// one, using the same key or context string if any.
    pub fn reset(&mut self) -> &mut Self {
        self.chunk_state = ChunkState::new(
            &self.key,
            0,
            self.chunk_state.flags,
            self.chunk_state.platform,
        );
        self.cv_stack.clear();
        self
    }
    fn merge_cv_stack(&mut self, chunk_counter: u64) {
        let post_merge_stack_len =
            (chunk_counter - self.initial_chunk_counter).count_ones() as usize;
        while self.cv_stack.len() > post_merge_stack_len {
            let right_child = self.cv_stack.pop().unwrap();
            let left_child = self.cv_stack.pop().unwrap();
            let parent_output = parent_node_output(
                &left_child,
                &right_child,
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
            self.cv_stack.push(parent_output.chaining_value());
        }
    }
    fn push_cv(&mut self, new_cv: &CVBytes, chunk_counter: u64) {
        self.merge_cv_stack(chunk_counter);
        self.cv_stack.push(*new_cv);
    }
    /// Add input bytes to the hash state. You can call this any number of times.
    ///
    /// This method is always single-threaded. For multithreading support, see
    /// [`update_rayon`](#method.update_rayon) (enabled with the `rayon` Cargo feature).
    ///
    /// Note that the degree of SIMD parallelism that `update` can use is limited by the size of
    /// this input buffer. See [`update_reader`](#method.update_reader).
    pub fn update(&mut self, input: &[u8]) -> &mut Self {
        self.update_with_join::<join::SerialJoin>(input)
    }
    fn update_with_join<J: join::Join>(&mut self, mut input: &[u8]) -> &mut Self {
        let input_offset = self.initial_chunk_counter * CHUNK_LEN as u64;
        if let Some(max) = hazmat::max_subtree_len(input_offset) {
            let remaining = max - self.count();
            assert!(
                input.len() as u64 <= remaining,
                "the subtree starting at {} contains at most {} bytes (found {})",
                CHUNK_LEN as u64 * self.initial_chunk_counter,
                max,
                input.len(),
            );
        }
        if self.chunk_state.count() > 0 {
            let want = CHUNK_LEN - self.chunk_state.count();
            let take = cmp::min(want, input.len());
            self.chunk_state.update(&input[..take]);
            input = &input[take..];
            if !input.is_empty() {
                debug_assert_eq!(self.chunk_state.count(), CHUNK_LEN);
                let chunk_cv = self.chunk_state.output().chaining_value();
                self.push_cv(&chunk_cv, self.chunk_state.chunk_counter);
                self.chunk_state = ChunkState::new(
                    &self.key,
                    self.chunk_state.chunk_counter + 1,
                    self.chunk_state.flags,
                    self.chunk_state.platform,
                );
            } else {
                return self;
            }
        }
        while input.len() > CHUNK_LEN {
            debug_assert_eq!(self.chunk_state.count(), 0, "no partial chunk data");
            debug_assert_eq!(CHUNK_LEN.count_ones(), 1, "power of 2 chunk len");
            let mut subtree_len = largest_power_of_two_leq(input.len());
            let count_so_far = self.chunk_state.chunk_counter * CHUNK_LEN as u64;
            while (subtree_len - 1) as u64 & count_so_far != 0 {
                subtree_len /= 2;
            }
            let subtree_chunks = (subtree_len / CHUNK_LEN) as u64;
            if subtree_len <= CHUNK_LEN {
                debug_assert_eq!(subtree_len, CHUNK_LEN);
                self.push_cv(
                    &ChunkState::new(
                        &self.key,
                        self.chunk_state.chunk_counter,
                        self.chunk_state.flags,
                        self.chunk_state.platform,
                    )
                    .update(&input[..subtree_len])
                    .output()
                    .chaining_value(),
                    self.chunk_state.chunk_counter,
                );
            } else {
                let cv_pair = compress_subtree_to_parent_node::<J>(
                    &input[..subtree_len],
                    &self.key,
                    self.chunk_state.chunk_counter,
                    self.chunk_state.flags,
                    self.chunk_state.platform,
                );
                let left_cv = array_ref!(cv_pair, 0, 32);
                let right_cv = array_ref!(cv_pair, 32, 32);
                self.push_cv(left_cv, self.chunk_state.chunk_counter);
                self.push_cv(
                    right_cv,
                    self.chunk_state.chunk_counter + (subtree_chunks / 2),
                );
            }
            self.chunk_state.chunk_counter += subtree_chunks;
            input = &input[subtree_len..];
        }
        debug_assert!(input.len() <= CHUNK_LEN);
        if !input.is_empty() {
            self.chunk_state.update(input);
            self.merge_cv_stack(self.chunk_state.chunk_counter);
        }
        self
    }
    fn final_output(&self) -> Output {
        if self.cv_stack.is_empty() {
            debug_assert_eq!(self.chunk_state.chunk_counter, self.initial_chunk_counter);
            return self.chunk_state.output();
        }
        let mut output: Output;
        let mut num_cvs_remaining = self.cv_stack.len();
        if self.chunk_state.count() > 0 {
            debug_assert_eq!(
                self.cv_stack.len(),
                (self.chunk_state.chunk_counter - self.initial_chunk_counter).count_ones() as usize,
                "cv stack does not need a merge",
            );
            output = self.chunk_state.output();
        } else {
            debug_assert!(self.cv_stack.len() >= 2);
            output = parent_node_output(
                &self.cv_stack[num_cvs_remaining - 2],
                &self.cv_stack[num_cvs_remaining - 1],
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
            num_cvs_remaining -= 2;
        }
        while num_cvs_remaining > 0 {
            output = parent_node_output(
                &self.cv_stack[num_cvs_remaining - 1],
                &output.chaining_value(),
                &self.key,
                self.chunk_state.flags,
                self.chunk_state.platform,
            );
            num_cvs_remaining -= 1;
        }
        output
    }
    /// Finalize the hash state and return the [`Hash`](struct.Hash.html) of
    /// the input.
    ///
    /// This method is idempotent. Calling it twice will give the same result.
    /// You can also add more input and finalize again.
    pub fn finalize(&self) -> Hash {
        assert_eq!(
            self.initial_chunk_counter, 0,
            "set_input_offset must be used with finalize_non_root",
        );
        self.final_output().root_hash()
    }
    /// Finalize the hash state and return an [`OutputReader`], which can
    /// supply any number of output bytes.
    ///
    /// This method is idempotent. Calling it twice will give the same result.
    /// You can also add more input and finalize again.
    ///
    /// [`OutputReader`]: struct.OutputReader.html
    pub fn finalize_xof(&self) -> OutputReader {
        assert_eq!(
            self.initial_chunk_counter, 0,
            "set_input_offset must be used with finalize_non_root",
        );
        OutputReader::new(self.final_output())
    }
    /// Return the total number of bytes hashed so far.
    ///
    /// [`hazmat::HasherExt::set_input_offset`] does not affect this value. This only counts bytes
    /// passed to [`update`](Hasher::update).
    pub fn count(&self) -> u64 {
        (self.chunk_state.chunk_counter - self.initial_chunk_counter) * CHUNK_LEN as u64
            + self.chunk_state.count() as u64
    }
    /// As [`update`](Hasher::update), but reading from a
    /// [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) implementation.
    ///
    /// [`Hasher`] implements
    /// [`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html), so it's possible to
    /// use [`std::io::copy`](https://doc.rust-lang.org/std/io/fn.copy.html) to update a [`Hasher`]
    /// from any reader. Unfortunately, this standard approach can limit performance, because
    /// `copy` currently uses an internal 8 KiB buffer that isn't big enough to take advantage of
    /// all SIMD instruction sets. (In particular, [AVX-512](https://en.wikipedia.org/wiki/AVX-512)
    /// needs a 16 KiB buffer.) `update_reader` avoids this performance problem and is slightly
    /// more convenient.
    ///
    /// The internal buffer size this method uses may change at any time, and it may be different
    /// for different targets. The only guarantee is that it will be large enough for all of this
    /// crate's SIMD implementations on the current platform.
    ///
    /// The most common implementer of
    /// [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) might be
    /// [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html), but note that memory
    /// mapping can be faster than this method for hashing large files. See
    /// [`update_mmap`](Hasher::update_mmap) and [`update_mmap_rayon`](Hasher::update_mmap_rayon),
    /// which require the `mmap` and (for the latter) `rayon` Cargo features.
    ///
    /// This method requires the `std` Cargo feature, which is enabled by default.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// # use std::io;
    /// # fn main() -> io::Result<()> {
    /// // Hash standard input.
    /// let mut hasher = blake3::Hasher::new();
    /// hasher.update_reader(std::io::stdin().lock())?;
    /// println!("{}", hasher.finalize());
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "std")]
    pub fn update_reader(&mut self, reader: impl std::io::Read) -> std::io::Result<&mut Self> {
        io::copy_wide(reader, self)?;
        Ok(self)
    }
    /// As [`update`](Hasher::update), but using Rayon-based multithreading
    /// internally.
    ///
    /// This method is gated by the `rayon` Cargo feature, which is disabled by
    /// default but enabled on [docs.rs](https://docs.rs).
    ///
    /// To get any performance benefit from multithreading, the input buffer
    /// needs to be large. As a rule of thumb on x86_64, `update_rayon` is
    /// _slower_ than `update` for inputs under 128 KiB. That threshold varies
    /// quite a lot across different processors, and it's important to benchmark
    /// your specific use case. See also the performance warning associated with
    /// [`update_mmap_rayon`](Hasher::update_mmap_rayon).
    ///
    /// If you already have a large buffer in memory, and you want to hash it
    /// with multiple threads, this method is a good option. However, reading a
    /// file into memory just to call this method can be a performance mistake,
    /// both because it requires lots of memory and because single-threaded
    /// reads can be slow. For hashing whole files, see
    /// [`update_mmap_rayon`](Hasher::update_mmap_rayon), which is gated by both
    /// the `rayon` and `mmap` Cargo features.
    #[cfg(feature = "rayon")]
    pub fn update_rayon(&mut self, input: &[u8]) -> &mut Self {
        self.update_with_join::<join::RayonJoin>(input)
    }
    /// As [`update`](Hasher::update), but reading the contents of a file using memory mapping.
    ///
    /// Not all files can be memory mapped, and memory mapping small files can be slower than
    /// reading them the usual way. In those cases, this method will fall back to standard file IO.
    /// The heuristic for whether to use memory mapping is currently very simple (file size >=
    /// 16 KiB), and it might change at any time.
    ///
    /// Like [`update`](Hasher::update), this method is single-threaded. In this author's
    /// experience, memory mapping improves single-threaded performance by ~10% for large files
    /// that are already in cache. This probably varies between platforms, and as always it's a
    /// good idea to benchmark your own use case. In comparison, the multithreaded
    /// [`update_mmap_rayon`](Hasher::update_mmap_rayon) method can have a much larger impact on
    /// performance.
    ///
    /// There's a correctness reason that this method takes
    /// [`Path`](https://doc.rust-lang.org/stable/std/path/struct.Path.html) instead of
    /// [`File`](https://doc.rust-lang.org/std/fs/struct.File.html): reading from a memory-mapped
    /// file ignores the seek position of the original file handle (it neither respects the current
    /// position nor updates the position). This difference in behavior would've caused
    /// `update_mmap` and [`update_reader`](Hasher::update_reader) to give different answers and
    /// have different side effects in some cases. Taking a
    /// [`Path`](https://doc.rust-lang.org/stable/std/path/struct.Path.html) avoids this problem by
    /// making it clear that a new [`File`](https://doc.rust-lang.org/std/fs/struct.File.html) is
    /// opened internally.
    ///
    /// This method requires the `mmap` Cargo feature, which is disabled by default but enabled on
    /// [docs.rs](https://docs.rs).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::io;
    /// # use std::path::Path;
    /// # fn main() -> io::Result<()> {
    /// let path = Path::new("file.dat");
    /// let mut hasher = blake3::Hasher::new();
    /// hasher.update_mmap(path)?;
    /// println!("{}", hasher.finalize());
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "mmap")]
    pub fn update_mmap(&mut self, path: impl AsRef<std::path::Path>) -> std::io::Result<&mut Self> {
        let file = std::fs::File::open(path.as_ref())?;
        if let Some(mmap) = io::maybe_mmap_file(&file)? {
            self.update(&mmap);
        } else {
            io::copy_wide(&file, self)?;
        }
        Ok(self)
    }
    /// As [`update_rayon`](Hasher::update_rayon), but reading the contents of a file using
    /// memory mapping. This is the default behavior of `b3sum`.
    ///
    /// For large files that are likely to be in cache, this can be much faster than
    /// single-threaded hashing. When benchmarks report that BLAKE3 is 10x or 20x faster than other
    /// cryptographic hashes, this is usually what they're measuring. However...
    ///
    /// **Performance Warning:** There are cases where multithreading hurts performance. The worst
    /// case is [a large file on a spinning disk](https://github.com/BLAKE3-team/BLAKE3/issues/31),
    /// where simultaneous reads from multiple threads can cause "thrashing" (i.e. the disk spends
    /// more time seeking around than reading data). Windows tends to be somewhat worse about this,
    /// in part because it's less likely than Linux to keep very large files in cache. More
    /// generally, if your CPU cores are already busy, then multithreading will add overhead
    /// without improving performance. If your code runs in different environments that you don't
    /// control and can't measure, then unfortunately there's no one-size-fits-all answer for
    /// whether multithreading is a good idea.
    ///
    /// The memory mapping behavior of this function is the same as
    /// [`update_mmap`](Hasher::update_mmap), and the heuristic for when to fall back to standard
    /// file IO might change at any time.
    ///
    /// This method requires both the `mmap` and `rayon` Cargo features, which are disabled by
    /// default but enabled on [docs.rs](https://docs.rs).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::io;
    /// # use std::path::Path;
    /// # fn main() -> io::Result<()> {
    /// # #[cfg(feature = "rayon")]
    /// # {
    /// let path = Path::new("big_file.dat");
    /// let mut hasher = blake3::Hasher::new();
    /// hasher.update_mmap_rayon(path)?;
    /// println!("{}", hasher.finalize());
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "mmap")]
    #[cfg(feature = "rayon")]
    pub fn update_mmap_rayon(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> std::io::Result<&mut Self> {
        let file = std::fs::File::open(path.as_ref())?;
        if let Some(mmap) = io::maybe_mmap_file(&file)? {
            self.update_rayon(&mmap);
        } else {
            io::copy_wide(&file, self)?;
        }
        Ok(self)
    }
}
