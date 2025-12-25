use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Rng {
    /// Creates a new random number generator with the initial seed.
    #[inline]
    #[must_use = "this creates a new instance of `Rng`; if you want to initialize the thread-local generator, use `fastrand::seed()` instead"]
    pub const fn with_seed(seed: u64) -> Self {
        Rng(seed)
    }
    /// Clones the generator by deterministically deriving a new generator based on the initial
    /// seed.
    ///
    /// This function can be used to create a new generator that is a "spinoff" of the old
    /// generator. The new generator will not produce the same sequence of values as the
    /// old generator.
    ///
    /// # Example
    ///
    /// ```
    /// // Seed two generators equally, and clone both of them.
    /// let mut base1 = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    /// base1.bool(); // Use the generator once.
    ///
    /// let mut base2 = fastrand::Rng::with_seed(0x4d595df4d0f33173);
    /// base2.bool(); // Use the generator once.
    ///
    /// let mut rng1 = base1.fork();
    /// let mut rng2 = base2.fork();
    ///
    /// println!("rng1 returns {}", rng1.u32(..));
    /// println!("rng2 returns {}", rng2.u32(..));
    /// ```
    #[inline]
    #[must_use = "this creates a new instance of `Rng`"]
    pub fn fork(&mut self) -> Self {
        Rng::with_seed(self.gen_u64())
    }
    /// Generates a random `char` in ranges a-z and A-Z.
    #[inline]
    pub fn alphabetic(&mut self) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        *self.choice(CHARS).unwrap() as char
    }
    /// Generates a random `char` in ranges a-z, A-Z and 0-9.
    #[inline]
    pub fn alphanumeric(&mut self) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        *self.choice(CHARS).unwrap() as char
    }
    /// Generates a random `bool`.
    #[inline]
    pub fn bool(&mut self) -> bool {
        self.u8(..) % 2 == 0
    }
    /// Generates a random digit in the given `base`.
    ///
    /// Digits are represented by `char`s in ranges 0-9 and a-z.
    ///
    /// Panics if the base is zero or greater than 36.
    #[inline]
    pub fn digit(&mut self, base: u32) -> char {
        if base == 0 {
            panic!("base cannot be zero");
        }
        if base > 36 {
            panic!("base cannot be larger than 36");
        }
        let num = self.u8(..base as u8);
        if num < 10 { (b'0' + num) as char } else { (b'a' + num - 10) as char }
    }
    /// Generates a random `f32` in range `0..=1`.
    #[inline]
    pub fn f32_inclusive(&mut self) -> f32 {
        const MUL: f32 = 1.0 / (1u64 << 63) as f32;
        (self.gen_u64() >> 1) as f32 * MUL
    }
    /// Generates a random `f32` in range `0..1`.
    ///
    /// Function `f32_inclusive()` is a little simpler and faster, so default
    /// to that if inclusive range is acceptable.
    #[inline]
    pub fn f32(&mut self) -> f32 {
        loop {
            let x = self.f32_inclusive();
            if x < 1.0 {
                return x;
            }
        }
    }
    /// Generates a random `f64` in range `0..=1`.
    #[inline]
    pub fn f64_inclusive(&mut self) -> f64 {
        const MUL: f64 = 1.0 / (1u64 << 63) as f64;
        (self.gen_u64() >> 1) as f64 * MUL
    }
    /// Generates a random `f64` in range `0..1`.
    ///
    /// Function `f64_inclusive()` is a little simpler and faster, so default
    /// to that if inclusive range is acceptable.
    #[inline]
    pub fn f64(&mut self) -> f64 {
        loop {
            let x = self.f64_inclusive();
            if x < 1.0 {
                return x;
            }
        }
    }
    /// Collects `amount` values at random from the iterable into a vector.
    ///
    /// The length of the returned vector equals `amount` unless the iterable
    /// contains insufficient elements, in which case it equals the number of
    /// elements available.
    ///
    /// Complexity is `O(n)` where `n` is the length of the iterable.
    #[cfg(feature = "alloc")]
    pub fn choose_multiple<I: IntoIterator>(
        &mut self,
        source: I,
        amount: usize,
    ) -> Vec<I::Item> {
        let mut reservoir = Vec::with_capacity(amount);
        let mut iter = source.into_iter();
        reservoir.extend(iter.by_ref().take(amount));
        if reservoir.len() == amount {
            for (i, elem) in iter.enumerate() {
                let end = i + 1 + amount;
                let k = self.usize(0..end);
                if let Some(slot) = reservoir.get_mut(k) {
                    *slot = elem;
                }
            }
        } else {
            if reservoir.capacity() > 3 * reservoir.len() {
                reservoir.shrink_to_fit();
            }
        }
        reservoir
    }
    rng_integer!(
        i8, u8, gen_u32, gen_mod_u32, "Generates a random `i8` in the given range."
    );
    rng_integer!(
        i16, u16, gen_u32, gen_mod_u32, "Generates a random `i16` in the given range."
    );
    rng_integer!(
        i32, u32, gen_u32, gen_mod_u32, "Generates a random `i32` in the given range."
    );
    rng_integer!(
        i64, u64, gen_u64, gen_mod_u64, "Generates a random `i64` in the given range."
    );
    rng_integer!(
        i128, u128, gen_u128, gen_mod_u128,
        "Generates a random `i128` in the given range."
    );
    #[cfg(target_pointer_width = "16")]
    rng_integer!(
        isize, usize, gen_u32, gen_mod_u32,
        "Generates a random `isize` in the given range."
    );
    #[cfg(target_pointer_width = "32")]
    rng_integer!(
        isize, usize, gen_u32, gen_mod_u32,
        "Generates a random `isize` in the given range."
    );
    #[cfg(target_pointer_width = "64")]
    rng_integer!(
        isize, usize, gen_u64, gen_mod_u64,
        "Generates a random `isize` in the given range."
    );
    /// Generates a random `char` in range a-z.
    #[inline]
    pub fn lowercase(&mut self) -> char {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        *self.choice(CHARS).unwrap() as char
    }
    /// Initializes this generator with the given seed.
    #[inline]
    pub fn seed(&mut self, seed: u64) {
        self.0 = seed;
    }
    /// Gives back **current** seed that is being held by this generator.
    #[inline]
    pub fn get_seed(&self) -> u64 {
        self.0
    }
    /// Choose an item from an iterator at random.
    ///
    /// This function may have an unexpected result if the `len()` property of the
    /// iterator does not match the actual number of items in the iterator. If
    /// the iterator is empty, this returns `None`.
    #[inline]
    pub fn choice<I>(&mut self, iter: I) -> Option<I::Item>
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
    {
        let mut iter = iter.into_iter();
        let len = iter.len();
        if len == 0 {
            return None;
        }
        let index = self.usize(0..len);
        iter.nth(index)
    }
    /// Shuffles a slice randomly.
    #[inline]
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in 1..slice.len() {
            slice.swap(i, self.usize(..=i));
        }
    }
    /// Fill a byte slice with random data.
    #[inline]
    pub fn fill(&mut self, slice: &mut [u8]) {
        let mut chunks = slice.chunks_exact_mut(core::mem::size_of::<u64>());
        for chunk in chunks.by_ref() {
            let n = self.gen_u64().to_ne_bytes();
            chunk.copy_from_slice(&n);
        }
        let remainder = chunks.into_remainder();
        if !remainder.is_empty() {
            let n = self.gen_u64().to_ne_bytes();
            remainder.copy_from_slice(&n[..remainder.len()]);
        }
    }
    rng_integer!(
        u8, u8, gen_u32, gen_mod_u32, "Generates a random `u8` in the given range."
    );
    rng_integer!(
        u16, u16, gen_u32, gen_mod_u32, "Generates a random `u16` in the given range."
    );
    rng_integer!(
        u32, u32, gen_u32, gen_mod_u32, "Generates a random `u32` in the given range."
    );
    rng_integer!(
        u64, u64, gen_u64, gen_mod_u64, "Generates a random `u64` in the given range."
    );
    rng_integer!(
        u128, u128, gen_u128, gen_mod_u128,
        "Generates a random `u128` in the given range."
    );
    #[cfg(target_pointer_width = "16")]
    rng_integer!(
        usize, usize, gen_u32, gen_mod_u32,
        "Generates a random `usize` in the given range."
    );
    #[cfg(target_pointer_width = "32")]
    rng_integer!(
        usize, usize, gen_u32, gen_mod_u32,
        "Generates a random `usize` in the given range."
    );
    #[cfg(target_pointer_width = "64")]
    rng_integer!(
        usize, usize, gen_u64, gen_mod_u64,
        "Generates a random `usize` in the given range."
    );
    /// Generates a random `char` in range A-Z.
    #[inline]
    pub fn uppercase(&mut self) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        *self.choice(CHARS).unwrap() as char
    }
    /// Generates a random `char` in the given range.
    ///
    /// Panics if the range is empty.
    #[inline]
    pub fn char(&mut self, range: impl RangeBounds<char>) -> char {
        let panic_empty_range = || {
            panic!("empty range: {:?}..{:?}", range.start_bound(), range.end_bound())
        };
        let surrogate_start = 0xd800u32;
        let surrogate_len = 0x800u32;
        let low = match range.start_bound() {
            Bound::Unbounded => 0u8 as char,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => {
                let scalar = if x as u32 == surrogate_start - 1 {
                    surrogate_start + surrogate_len
                } else {
                    x as u32 + 1
                };
                char::try_from(scalar).unwrap_or_else(|_| panic_empty_range())
            }
        };
        let high = match range.end_bound() {
            Bound::Unbounded => core::char::MAX,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => {
                let scalar = if x as u32 == surrogate_start + surrogate_len {
                    surrogate_start - 1
                } else {
                    (x as u32).wrapping_sub(1)
                };
                char::try_from(scalar).unwrap_or_else(|_| panic_empty_range())
            }
        };
        if low > high {
            panic_empty_range();
        }
        let gap = if (low as u32) < surrogate_start && (high as u32) >= surrogate_start {
            surrogate_len
        } else {
            0
        };
        let range = high as u32 - low as u32 - gap;
        let mut val = self.u32(0..=range) + low as u32;
        if val >= surrogate_start {
            val += gap;
        }
        val.try_into().unwrap()
    }
}
