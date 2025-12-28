macro_rules! global_fn_name {
    () => {
        pub fn global_fn_name (base : Symbol) -> String { format ! ("__rust_{base}") }
    };
}

global_fn_name!()