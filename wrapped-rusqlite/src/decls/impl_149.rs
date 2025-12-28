macro_rules! deps {
    () => {
        InnerConnection!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl Drop for InnerConnection { # [expect (unused_must_use)] # [inline] fn drop (& mut self) { self . close () ; } }
    };
}

impl_149!();