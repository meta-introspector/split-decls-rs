macro_rules! default_fn_name {
    () => {
        pub fn default_fn_name (base : Symbol) -> String { format ! ("__rdl_{base}") }
    };
}

default_fn_name!()