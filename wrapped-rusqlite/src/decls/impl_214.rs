macro_rules! deps {
    () => {
        Rows!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl Drop for Rows < '_ > { # [expect (unused_must_use)] # [inline] fn drop (& mut self) { self . reset () ; } }
    };
}

impl_214!();