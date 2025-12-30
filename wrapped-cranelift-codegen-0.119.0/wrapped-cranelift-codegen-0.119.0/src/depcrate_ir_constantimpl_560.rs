// Generated macro for impl_560 (impl)
macro_rules! Depcrate_ir_constantimpl_560 {
() => {
// Module: crate::ir::constant
// Provides: {"impl_560"}
// Dependencies: {}
impl FromIterator < u8 > for ConstantData { fn from_iter < T : IntoIterator < Item = u8 > > (iter : T) -> Self { let v = iter . into_iter () . collect () ; Self (v) } }
};
}
