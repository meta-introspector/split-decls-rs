macro_rules! deps {
    () => {
        Dependencies!();
        Signature!();
        TypeMap!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl Dependencies for Signature { fn combine (& self , dependencies : & mut TypeMap) { self . types () . for_each (| ty | ty . combine (dependencies)) ; } }
    };
}

impl_66!()