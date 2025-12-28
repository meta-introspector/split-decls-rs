macro_rules! deps {
    () => {
        AsLockedWrite!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl AsLockedWrite for dyn std :: io :: Write + Send + Sync { type Write < 'w > = & 'w mut Self ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self } }
    };
}

impl_79!()