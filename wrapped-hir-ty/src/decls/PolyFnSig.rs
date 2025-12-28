macro_rules! deps {
    () => {
        FnSig!();
        Binder!();
    };
}

macro_rules! PolyFnSig {
    () => {
        deps!();
        pub type PolyFnSig < 'db > = Binder < 'db , rustc_type_ir :: FnSig < DbInterner < 'db > > > ;
    };
}

PolyFnSig!()