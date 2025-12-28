macro_rules! deps {
    () => {
        RecoveryId!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl From < RecoveryId > for u8 { fn from (id : RecoveryId) -> u8 { id . 0 } }
    };
}

impl_5!()