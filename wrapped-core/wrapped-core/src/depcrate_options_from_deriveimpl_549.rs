// Generated macro for impl_549 (impl)
macro_rules! Depcrate_options_from_deriveimpl_549 {
() => {
// Module: crate::options::from_derive
// Provides: {"impl_549"}
// Dependencies: {}
impl FdiOptions { pub fn new (di : & syn :: DeriveInput) -> Result < Self > { (FdiOptions { base : OuterFrom :: start (di) ? , vis : Default :: default () , generics : Default :: default () , data : Default :: default () , supports : Default :: default () , }) . parse_attributes (& di . attrs) ? . parse_body (& di . data) } }
};
}
