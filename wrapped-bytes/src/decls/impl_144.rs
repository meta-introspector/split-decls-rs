macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Drop for Shared { fn drop (& mut self) { unsafe { dealloc (self . buf , Layout :: from_size_align (self . cap , 1) . unwrap ()) } } }
    };
}

impl_144!();