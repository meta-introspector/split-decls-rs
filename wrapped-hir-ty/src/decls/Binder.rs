macro_rules! Binder {
    () => {
        pub type Binder < 'db , T > = rustc_type_ir :: Binder < DbInterner < 'db > , T > ;
    };
}

Binder!();