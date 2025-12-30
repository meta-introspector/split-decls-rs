// Generated macro for impl_535 (impl)
macro_rules! Depcrate_options_from_attributesimpl_535 {
() => {
// Module: crate::options::from_attributes
// Provides: {"impl_535"}
// Dependencies: {}
impl FromAttributesOptions { pub fn new (di : & syn :: DeriveInput) -> Result < Self > { let opts = (Self { base : OuterFrom :: start (di) ? , }) . parse_attributes (& di . attrs) ? . parse_body (& di . data) ? ; if ! opts . is_newtype () && opts . base . attr_names . is_empty () { Err (Error :: custom ("FromAttributes without attributes collects nothing" ,)) } else { Ok (opts) } } fn is_newtype (& self) -> bool { if let Data :: Struct (ref data) = self . base . container . data { data . is_newtype () } else { false } } }
};
}
