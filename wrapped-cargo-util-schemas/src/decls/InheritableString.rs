macro_rules! deps {
    () => {
        InheritableField!();
    };
}

macro_rules! InheritableString {
    () => {
        deps!();
        pub type InheritableString = InheritableField < String > ;
    };
}

InheritableString!()