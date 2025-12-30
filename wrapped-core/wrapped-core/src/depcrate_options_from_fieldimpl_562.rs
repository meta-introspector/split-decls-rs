// Generated macro for impl_562 (impl)
macro_rules! Depcrate_options_from_fieldimpl_562 {
() => {
// Module: crate::options::from_field
// Provides: {"impl_562"}
// Dependencies: {}
impl FromFieldOptions { pub fn new (di : & syn :: DeriveInput) -> Result < Self > { (FromFieldOptions { base : OuterFrom :: start (di) ? , vis : Default :: default () , ty : Default :: default () , }) . parse_attributes (& di . attrs) ? . parse_body (& di . data) } }
};
}
