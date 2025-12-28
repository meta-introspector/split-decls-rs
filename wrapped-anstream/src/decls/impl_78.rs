macro_rules! deps {
    () => {
        AsLockedWrite!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl AsLockedWrite for dyn std :: io :: Write + Send { type Write < 'w > = & 'w mut Self ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self } }
    };
}

impl_78!();