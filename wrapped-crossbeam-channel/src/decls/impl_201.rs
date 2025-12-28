macro_rules! deps {
    () => {
        SyncWaker!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl Drop for SyncWaker { # [inline] fn drop (& mut self) { debug_assert ! (self . is_empty . load (Ordering :: SeqCst)) ; } }
    };
}

impl_201!();