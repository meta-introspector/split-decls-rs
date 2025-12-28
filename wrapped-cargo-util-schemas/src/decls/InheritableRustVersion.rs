macro_rules! deps {
    () => {
        InheritableField!();
        RustVersion!();
    };
}

macro_rules! InheritableRustVersion {
    () => {
        deps!();
        pub type InheritableRustVersion = InheritableField < RustVersion > ;
    };
}

InheritableRustVersion!()