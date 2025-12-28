macro_rules! CanonicalVarKind {
    () => {
        pub type CanonicalVarKind < 'db > = rustc_type_ir :: CanonicalVarKind < DbInterner < 'db > > ;
    };
}

CanonicalVarKind!()