macro_rules! deps {
    () => {
        InheritableField!();
        StringOrBool!();
    };
}

macro_rules! InheritableStringOrBool {
    () => {
        deps!();
        pub type InheritableStringOrBool = InheritableField < StringOrBool > ;
    };
}

InheritableStringOrBool!()