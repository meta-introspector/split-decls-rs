macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! impl_703 {
    () => {
        deps!();
        impl Drop for Connection { # [inline] fn drop (& mut self) { # [cfg (feature = "cache")] self . flush_prepared_statement_cache () ; } }
    };
}

impl_703!()