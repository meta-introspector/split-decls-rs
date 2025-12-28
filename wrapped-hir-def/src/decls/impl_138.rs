macro_rules! deps {
    () => {
        KeyMap!();
        Policy!();
        Key!();
        DynMap!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < P : Policy > Index < Key < P :: K , P :: V , P > > for DynMap { type Output = KeyMap < Key < P :: K , P :: V , P > > ; fn index (& self , _key : Key < P :: K , P :: V , P >) -> & Self :: Output { unsafe { std :: mem :: transmute :: < & DynMap , & KeyMap < Key < P :: K , P :: V , P > > > (self) } } }
    };
}

impl_138!()