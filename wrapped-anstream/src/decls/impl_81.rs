macro_rules! deps {
    () => {
        AsLockedWrite!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl AsLockedWrite for std :: fs :: File { type Write < 'w > = & 'w mut Self ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self } }
    };
}

impl_81!();