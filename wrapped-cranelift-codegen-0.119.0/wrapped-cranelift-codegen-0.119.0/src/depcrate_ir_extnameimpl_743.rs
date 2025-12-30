// Generated macro for impl_743 (impl)
macro_rules! Depcrate_ir_extnameimpl_743 {
() => {
// Module: crate::ir::extname
// Provides: {"impl_743"}
// Dependencies: {}
impl FromStr for ExternalName { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { if let Ok (ks) = s . parse () { return Ok (Self :: KnownSymbol (ks)) ; } if let Ok (lc) = s . parse () { return Ok (Self :: LibCall (lc)) ; } Ok (Self :: testcase (s . as_bytes ())) } }
};
}
