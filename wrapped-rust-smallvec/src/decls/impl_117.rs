macro_rules! deps {
    () => {
        DropDealloc!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl Drop for DropDealloc { # [inline] fn drop (& mut self) { unsafe { if self . size_bytes > 0 { alloc :: alloc :: dealloc (self . ptr . as_ptr () , Layout :: from_size_align_unchecked (self . size_bytes , self . align) ,) ; } } } }
    };
}

impl_117!();