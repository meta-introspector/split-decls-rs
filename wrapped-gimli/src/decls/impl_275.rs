macro_rules! deps {
    () => {
        Unit!();
        UnitRef!();
        Reader!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl < 'a , R : Reader > core :: ops :: Deref for UnitRef < 'a , R > { type Target = & 'a Unit < R > ; fn deref (& self) -> & Self :: Target { & self . unit } }
    };
}

impl_275!();