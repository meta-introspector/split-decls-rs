macro_rules! deps {
    () => {
        Statement!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl Drop for Statement < '_ > { # [expect (unused_must_use)] # [inline] fn drop (& mut self) { self . finalize_ () ; } }
    };
}

impl_277!()