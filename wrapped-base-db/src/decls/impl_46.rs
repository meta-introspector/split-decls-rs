macro_rules! deps {
    () => {
        DependencyBuilder!();
        CrateBuilderId!();
        CrateName!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl DependencyBuilder { pub fn new (name : CrateName , crate_id : CrateBuilderId) -> Self { Self { name , crate_id , prelude : true , sysroot : false } } pub fn with_prelude (name : CrateName , crate_id : CrateBuilderId , prelude : bool , sysroot : bool ,) -> Self { Self { name , crate_id , prelude , sysroot } } }
    };
}

impl_46!();