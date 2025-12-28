macro_rules! deps {
    () => {
        AsLockedWrite!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl AsLockedWrite for dyn std :: io :: Write { type Write < 'w > = & 'w mut Self ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self } }
    };
}

impl_77!();