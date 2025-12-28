macro_rules! Edge {
    () => {
        type Edge < 'a > = (Crate , & 'a BuiltDependency) ;
    };
}

Edge!()