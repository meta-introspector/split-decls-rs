macro_rules! deps {
    () => {
        SectionMut!();
        Section!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'event > Deref for SectionMut < '_ , 'event > { type Target = file :: Section < 'event > ; fn deref (& self) -> & Self :: Target { self . section } }
    };
}

impl_7!()