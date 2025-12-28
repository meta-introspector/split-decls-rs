macro_rules! deps {
    () => {
        AsName!();
        Name!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl AsName for base_db :: BuiltDependency { fn as_name (& self) -> Name { Name :: new_symbol_root ((* self . name) . clone ()) } }
    };
}

impl_156!();