macro_rules! deps {
    () => {
        SealedBag!();
        Epoch!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl SealedBag { # [doc = " Checks if it is safe to drop the bag w.r.t. the given global epoch."] fn is_expired (& self , global_epoch : Epoch) -> bool { global_epoch . wrapping_sub (self . epoch) >= 2 } }
    };
}

impl_109!()