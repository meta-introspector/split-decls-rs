macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl TryFrom < u8 > for State { type Error = u8 ; # [inline (always)] fn try_from (raw : u8) -> Result < Self , Self :: Error > { STATES . get (raw as usize) . ok_or (raw) . copied () } }
    };
}

impl_16!()