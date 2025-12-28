macro_rules! deps {
    () => {
        TomlTarget!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl TomlTarget { pub fn new () -> TomlTarget { TomlTarget :: default () } pub fn proc_macro (& self) -> Option < bool > { self . proc_macro . or (self . proc_macro2) . or_else (| | { if let Some (types) = self . crate_types () { if types . contains (& "proc-macro" . to_string ()) { return Some (true) ; } } None }) } pub fn crate_types (& self) -> Option < & Vec < String > > { self . crate_type . as_ref () . or_else (| | self . crate_type2 . as_ref ()) } }
    };
}

impl_150!();