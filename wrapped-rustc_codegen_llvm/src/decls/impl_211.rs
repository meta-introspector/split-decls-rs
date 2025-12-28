macro_rules! deps {
    () => {
        GenericCx!();
        SCx!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < 'll , T : Borrow < SCx < 'll > > > Deref for GenericCx < 'll , T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_211!()