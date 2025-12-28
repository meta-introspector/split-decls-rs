macro_rules! deps {
    () => {
        Transport!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl Drop for Transport { fn drop (& mut self) { if self . owned { unsafe { (* self . raw) . free . unwrap () (self . raw) } } } }
    };
}

impl_176!();