macro_rules! deps {
    () => {
        AsLockedWrite!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl AsLockedWrite for std :: io :: StderrLock < 'static > { type Write < 'w > = & 'w mut Self ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self } }
    };
}

impl_76!();