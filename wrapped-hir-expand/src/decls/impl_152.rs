macro_rules! deps {
    () => {
        AsName!();
        Name!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl AsName for ast :: Name { fn as_name (& self) -> Name { Name :: new_root (& self . text ()) } }
    };
}

impl_152!()