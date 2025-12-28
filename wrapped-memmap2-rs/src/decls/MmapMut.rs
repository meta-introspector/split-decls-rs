macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! MmapMut {
    () => {
        deps!();
        # [doc = " A handle to a mutable memory mapped buffer."] # [doc = ""] # [doc = " A file-backed `MmapMut` buffer may be used to read from or write to a file. An anonymous"] # [doc = " `MmapMut` buffer may be used any place that an in-memory byte buffer is needed. Use"] # [doc = " [`MmapMut::map_mut()`] and [`MmapMut::map_anon()`] to create a mutable memory map of the"] # [doc = " respective types, or [`MmapOptions::map_mut()`] and [`MmapOptions::map_anon()`] if non-default"] # [doc = " options are required."] # [doc = ""] # [doc = " A file backed `MmapMut` is created by `&File` reference, and will remain valid even after the"] # [doc = " `File` is dropped. In other words, the `MmapMut` handle is completely independent of the `File`"] # [doc = " used to create it. For consistency, on some platforms this is achieved by duplicating the"] # [doc = " underlying file handle. The memory will be unmapped when the `MmapMut` handle is dropped."] # [doc = ""] # [doc = " Dereferencing and accessing the bytes of the buffer may result in page faults (e.g. swapping"] # [doc = " the mapped pages into physical memory) though the details of this are platform specific."] # [doc = ""] # [doc = " `MmapMut` is [`Sync`] and [`Send`]."] # [doc = ""] # [doc = " See [`Mmap`] for the immutable version."] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " All file-backed memory map constructors are marked `unsafe` because of the potential for"] # [doc = " *Undefined Behavior* (UB) using the map if the underlying file is subsequently modified, in or"] # [doc = " out of process. Applications must consider the risk and take appropriate precautions when using"] # [doc = " file-backed maps. Solutions such as file permissions, locks or process-private (e.g. unlinked)"] # [doc = " files exist but are platform specific and limited."] pub struct MmapMut { inner : MmapInner , }
    };
}

MmapMut!()