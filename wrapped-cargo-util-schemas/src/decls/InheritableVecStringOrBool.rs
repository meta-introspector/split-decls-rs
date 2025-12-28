macro_rules! deps {
    () => {
        VecStringOrBool!();
        InheritableField!();
    };
}

macro_rules! InheritableVecStringOrBool {
    () => {
        deps!();
        pub type InheritableVecStringOrBool = InheritableField < VecStringOrBool > ;
    };
}

InheritableVecStringOrBool!()