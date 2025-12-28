macro_rules! deps {
    () => {
        Powerset!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        impl < I : Iterator > Powerset < I > { # [doc = " Returns true if `k` has been incremented, false otherwise."] fn increment_k (& mut self) -> bool { if self . combs . k () < self . combs . n () || self . combs . k () == 0 { self . combs . reset (self . combs . k () + 1) ; true } else { false } } }
    };
}

impl_440!()