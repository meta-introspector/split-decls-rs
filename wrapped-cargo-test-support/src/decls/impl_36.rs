macro_rules! deps {
    () => {
        ContainerHandle!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Drop for ContainerHandle { fn drop (& mut self) { if std :: env :: var_os ("CARGO_CONTAINER_TEST_KEEP") . is_some () { return ; } remove_if_exists (& self . name) ; } }
    };
}

impl_36!();