macro_rules! deps {
    () => {
        AsLockedWrite!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl AsLockedWrite for std :: io :: StdoutLock < 'static > { type Write < 'w > = & 'w mut Self ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self } }
    };
}

impl_74!()