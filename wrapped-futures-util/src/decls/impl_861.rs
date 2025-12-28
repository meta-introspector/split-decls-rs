macro_rules! deps {
    () => {
        Task!();
    };
}

macro_rules! impl_861 {
    () => {
        deps!();
        impl < Fut > Drop for Task < Fut > { fn drop (& mut self) { unsafe { if (* self . future . get ()) . is_some () { abort ("future still here when dropping") ; } } } }
    };
}

impl_861!();