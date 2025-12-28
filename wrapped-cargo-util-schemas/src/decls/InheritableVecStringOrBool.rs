macro_rules! deps {
    () => {
        InheritableField!();
        VecStringOrBool!();
    };
}

macro_rules! InheritableVecStringOrBool {
    () => {
        deps!();
        pub type InheritableVecStringOrBool = InheritableField < VecStringOrBool > ;
    };
}

InheritableVecStringOrBool!();