macro_rules! EarlyBinder {
    () => {
        pub type EarlyBinder < 'db , T > = rustc_type_ir :: EarlyBinder < DbInterner < 'db > , T > ;
    };
}

EarlyBinder!()