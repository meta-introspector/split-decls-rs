macro_rules! deps {
    () => {
        Stdout!();
        AsLockedWrite!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl AsLockedWrite for std :: io :: Stdout { type Write < 'w > = std :: io :: StdoutLock < 'w > ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self . lock () } }
    };
}

impl_73!();