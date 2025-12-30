// Generated macro for impl_471 (impl)
macro_rules! Depcrate_attrsimpl_471 {
() => {
// Module: crate::attrs
// Provides: {"impl_471"}
// Dependencies: {}
impl EarlyLintPass for EarlyAttributes { fn check_attribute (& mut self , cx : & EarlyContext < '_ > , attr : & Attribute) { deprecated_cfg_attr :: check (cx , attr , & self . msrv) ; deprecated_cfg_attr :: check_clippy (cx , attr) ; non_minimal_cfg :: check (cx , attr) ; } extract_msrv_attr ! () ; }
};
}
