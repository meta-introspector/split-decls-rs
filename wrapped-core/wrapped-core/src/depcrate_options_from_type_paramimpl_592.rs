// Generated macro for impl_592 (impl)
macro_rules! Depcrate_options_from_type_paramimpl_592 {
() => {
// Module: crate::options::from_type_param
// Provides: {"impl_592"}
// Dependencies: {}
impl FromTypeParamOptions { pub fn new (di : & syn :: DeriveInput) -> Result < Self > { (FromTypeParamOptions { base : OuterFrom :: start (di) ? , bounds : None , default : None , }) . parse_attributes (& di . attrs) ? . parse_body (& di . data) } }
};
}
