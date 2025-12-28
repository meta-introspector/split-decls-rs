macro_rules! deps {
    () => {
        Policy!();
        KeyMap!();
        DynMap!();
        Key!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < P : Policy > IndexMut < Key < P :: K , P :: V , P > > for DynMap { fn index_mut (& mut self , _key : Key < P :: K , P :: V , P >) -> & mut Self :: Output { unsafe { std :: mem :: transmute :: < & mut DynMap , & mut KeyMap < Key < P :: K , P :: V , P > > > (self) } } }
    };
}

impl_139!();