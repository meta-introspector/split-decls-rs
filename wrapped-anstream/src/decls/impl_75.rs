macro_rules! deps {
    () => {
        AsLockedWrite!();
        Stderr!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl AsLockedWrite for std :: io :: Stderr { type Write < 'w > = std :: io :: StderrLock < 'w > ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self . lock () } }
    };
}

impl_75!()