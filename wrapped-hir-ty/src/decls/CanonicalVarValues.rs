macro_rules! CanonicalVarValues {
    () => {
        pub type CanonicalVarValues < 'db > = rustc_type_ir :: CanonicalVarValues < DbInterner < 'db > > ;
    };
}

CanonicalVarValues!()