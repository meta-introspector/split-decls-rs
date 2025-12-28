macro_rules! CanonicalQueryInput {
    () => {
        pub type CanonicalQueryInput < 'db , V > = rustc_type_ir :: CanonicalQueryInput < DbInterner < 'db > , V > ;
    };
}

CanonicalQueryInput!()