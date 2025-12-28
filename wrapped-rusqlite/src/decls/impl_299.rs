macro_rules! deps {
    () => {
        Transaction!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        # [expect (unused_must_use)] impl Drop for Transaction < '_ > { # [inline] fn drop (& mut self) { self . finish_ () ; } }
    };
}

impl_299!()