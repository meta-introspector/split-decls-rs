macro_rules! deps {
    () => {
        Savepoint!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        # [expect (unused_must_use)] impl Drop for Savepoint < '_ > { # [inline] fn drop (& mut self) { self . finish_ () ; } }
    };
}

impl_302!();