macro_rules! diagnostics_registry {
    () => {
        pub fn diagnostics_registry () -> Registry { Registry :: new (rustc_errors :: codes :: DIAGNOSTICS) }
    };
}

diagnostics_registry!()