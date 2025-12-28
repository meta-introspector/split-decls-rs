macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Drop for Mmap { fn drop (& mut self) { unsafe { libc :: munmap (self . addr . as_ptr () , self . len) ; } } }
    };
}

impl_3!();