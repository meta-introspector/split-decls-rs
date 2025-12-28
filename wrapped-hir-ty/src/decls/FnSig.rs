macro_rules! FnSig {
    () => {
        pub type FnSig < 'db > = rustc_type_ir :: FnSig < DbInterner < 'db > > ;
    };
}

FnSig!();