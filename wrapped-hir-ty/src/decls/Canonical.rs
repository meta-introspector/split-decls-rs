macro_rules! Canonical {
    () => {
        pub type Canonical < 'db , T > = rustc_type_ir :: Canonical < DbInterner < 'db > , T > ;
    };
}

Canonical!()