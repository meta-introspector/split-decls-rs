macro_rules! deps {
    () => {
        ParkResult!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl ParkResult { # [doc = " Returns true if we were unparked by another thread."] # [inline] pub fn is_unparked (self) -> bool { if let ParkResult :: Unparked (_) = self { true } else { false } } }
    };
}

impl_25!()