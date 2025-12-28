macro_rules! Mmap {
    () => {
        # [doc = " A region of memory mapped using `mmap(2)`."] pub (crate) struct Mmap { addr : ptr :: NonNull < libc :: c_void > , len : usize , }
    };
}

Mmap!()