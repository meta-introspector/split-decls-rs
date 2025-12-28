macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl TryFrom < u8 > for Action { type Error = u8 ; # [inline (always)] fn try_from (raw : u8) -> Result < Self , Self :: Error > { ACTIONS . get (raw as usize) . ok_or (raw) . copied () } }
    };
}

impl_19!()