macro_rules! deps {
    () => {
        AsLockedWrite!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl AsLockedWrite for Vec < u8 > { type Write < 'w > = & 'w mut Self ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self } }
    };
}

impl_80!()