macro_rules! deps {
    () => {
        GenericCx!();
        SCx!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < 'll , T : Borrow < SCx < 'll > > > DerefMut for GenericCx < 'll , T > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_212!();