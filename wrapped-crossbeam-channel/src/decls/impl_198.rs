macro_rules! deps {
    () => {
        Waker!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl Drop for Waker { # [inline] fn drop (& mut self) { debug_assert_eq ! (self . selectors . len () , 0) ; debug_assert_eq ! (self . observers . len () , 0) ; } }
    };
}

impl_198!()