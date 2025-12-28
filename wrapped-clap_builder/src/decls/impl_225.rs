macro_rules! impl_225 {
    () => {
        impl PartialEq for Inner { fn eq (& self , other : & Inner) -> bool { self . as_str () == other . as_str () } }
    };
}

impl_225!();