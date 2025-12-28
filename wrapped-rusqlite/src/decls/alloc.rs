macro_rules! deps {
    () => {
        SqliteMallocString!();
    };
}

macro_rules! alloc {
    () => {
        deps!();
        pub (crate) fn alloc (s : & str) -> * mut c_char { SqliteMallocString :: from_str (s) . into_raw () }
    };
}

alloc!()