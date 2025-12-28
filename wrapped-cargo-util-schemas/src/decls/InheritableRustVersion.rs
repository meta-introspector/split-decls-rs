macro_rules! deps {
    () => {
        RustVersion!();
        InheritableField!();
    };
}

macro_rules! InheritableRustVersion {
    () => {
        deps!();
        pub type InheritableRustVersion = InheritableField < RustVersion > ;
    };
}

InheritableRustVersion!();