macro_rules! deps {
    () => {
        Name!();
        AsName!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < Span > AsName for tt :: Ident < Span > { fn as_name (& self) -> Name { Name :: new_root (self . sym . as_str ()) } }
    };
}

impl_154!()