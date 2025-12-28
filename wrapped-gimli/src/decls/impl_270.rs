macro_rules! deps {
    () => {
        Reader!();
        Unit!();
        UnitHeader!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < 'a , R : Reader > core :: ops :: Deref for Unit < R > { type Target = UnitHeader < R > ; fn deref (& self) -> & Self :: Target { & self . header } }
    };
}

impl_270!()