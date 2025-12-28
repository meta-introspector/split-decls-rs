macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! impl_1264 {
    () => {
        deps!();
        impl < T > Drop for Inner < T > { fn drop (& mut self) { assert ! (self . state . load (SeqCst) . is_null ()) ; } }
    };
}

impl_1264!();