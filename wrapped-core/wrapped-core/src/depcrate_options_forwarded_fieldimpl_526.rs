// Generated macro for impl_526 (impl)
macro_rules! Depcrate_options_forwarded_fieldimpl_526 {
() => {
// Module: crate::options::forwarded_field
// Provides: {"impl_526"}
// Dependencies: {}
impl FromField for ForwardedField { fn from_field (field : & syn :: Field) -> crate :: Result < Self > { let result = Self { ident : field . ident . clone () . ok_or_else (| | { Error :: custom ("forwarded field must be named field") . with_span (field) }) ? , with : None , } ; result . parse_attributes (& field . attrs) } }
};
}
