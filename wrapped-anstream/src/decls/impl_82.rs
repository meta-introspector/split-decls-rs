macro_rules! deps {
    () => {
        Buffer!();
        AsLockedWrite!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        # [allow (deprecated)] impl AsLockedWrite for crate :: Buffer { type Write < 'w > = & 'w mut Self ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self } }
    };
}

impl_82!();