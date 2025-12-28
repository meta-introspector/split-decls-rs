macro_rules! deps {
    () => {
        Interface!();
        ScopedInterface!();
        ScopedHeap!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl < T : Interface > Drop for ScopedInterface < '_ , T > { fn drop (& mut self) { unsafe { let _ = Box :: from_raw (self . interface . as_raw () as * const _ as * mut ScopedHeap) ; } } }
    };
}

impl_196!()