macro_rules! deps {
    () => {
        FieldDef!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl FieldDef < '_ > { pub fn is_positional (& self) -> bool { self . ident . as_str () . as_bytes () [0] . is_ascii_digit () } }
    };
}

impl_306!();