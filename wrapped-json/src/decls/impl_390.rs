macro_rules! deps {
    () => {
        Limb!();
        Bigint!();
        Math!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl Math for Bigint { # [inline] fn data (& self) -> & Vec < Limb > { & self . data } # [inline] fn data_mut (& mut self) -> & mut Vec < Limb > { & mut self . data } }
    };
}

impl_390!()