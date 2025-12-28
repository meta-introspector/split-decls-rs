macro_rules! deps {
    () => {
        InheritableField!();
    };
}

macro_rules! InheritableVecString {
    () => {
        deps!();
        pub type InheritableVecString = InheritableField < Vec < String > > ;
    };
}

InheritableVecString!()