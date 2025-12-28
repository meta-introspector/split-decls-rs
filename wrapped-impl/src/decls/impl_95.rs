macro_rules! deps {
    () => {
        IdentUnraw!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl IdentUnraw { pub fn new (ident : Ident) -> Self { IdentUnraw (ident) } pub fn to_local (& self) -> Ident { let unraw = self . 0 . unraw () ; let repr = unraw . to_string () ; if syn :: parse_str :: < Ident > (& repr) . is_err () { if let "_" | "super" | "self" | "Self" | "crate" = repr . as_str () { } else { return Ident :: new_raw (& repr , Span :: call_site ()) ; } } unraw } pub fn set_span (& mut self , span : Span) { self . 0 . set_span (span) ; } }
    };
}

impl_95!();