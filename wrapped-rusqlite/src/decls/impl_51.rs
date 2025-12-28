macro_rules! deps {
    () => {
        Blob!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        # [expect (unused_must_use)] impl Drop for Blob < '_ > { # [inline] fn drop (& mut self) { self . close_ () ; } }
    };
}

impl_51!()