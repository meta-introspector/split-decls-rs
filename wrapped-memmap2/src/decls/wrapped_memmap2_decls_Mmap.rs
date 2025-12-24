use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A handle to an immutable memory mapped buffer.
///
/// A `Mmap` may be backed by a file, or it can be anonymous map, backed by volatile memory. Use
/// [`MmapOptions`] or [`map()`] to create a file-backed memory map. To create an immutable
/// anonymous memory map, first create a mutable anonymous memory map, and then make it immutable
/// with [`MmapMut::make_read_only()`].
///
/// A file backed `Mmap` is created by `&File` reference, and will remain valid even after the
/// `File` is dropped. In other words, the `Mmap` handle is completely independent of the `File`
/// used to create it. For consistency, on some platforms this is achieved by duplicating the
/// underlying file handle. The memory will be unmapped when the `Mmap` handle is dropped.
///
/// Dereferencing and accessing the bytes of the buffer may result in page faults (e.g. swapping
/// the mapped pages into physical memory) though the details of this are platform specific.
///
/// `Mmap` is [`Sync`] and [`Send`].
///
/// See [`MmapMut`] for the mutable version.
///
/// ## Safety
///
/// All file-backed memory map constructors are marked `unsafe` because of the potential for
/// *Undefined Behavior* (UB) using the map if the underlying file is subsequently modified, in or
/// out of process. Applications must consider the risk and take appropriate precautions when using
/// file-backed maps. Solutions such as file permissions, locks or process-private (e.g. unlinked)
/// files exist but are platform specific and limited.
///
/// ## Example
///
/// ```
/// use memmap2::MmapOptions;
/// use std::io::Write;
/// use std::fs::File;
///
/// # fn main() -> std::io::Result<()> {
/// let file = File::open("README.md")?;
/// let mmap = unsafe { MmapOptions::new().map(&file)? };
/// assert_eq!(b"# memmap2", &mmap[0..9]);
/// # Ok(())
/// # }
/// ```
///
/// [`map()`]: Mmap::map()
pub struct Mmap {
    inner: MmapInner,
}
