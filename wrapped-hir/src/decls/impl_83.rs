macro_rules! deps {
    () => {
        SemanticsImpl!();
        Semantics!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < 'db , DB : ? Sized > ops :: Deref for Semantics < 'db , DB > { type Target = SemanticsImpl < 'db > ; fn deref (& self) -> & Self :: Target { & self . imp } }
    };
}

impl_83!()