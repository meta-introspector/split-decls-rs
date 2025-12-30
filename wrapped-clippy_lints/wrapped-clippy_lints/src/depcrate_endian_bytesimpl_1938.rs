// Generated macro for impl_1938 (impl)
macro_rules! Depcrate_endian_bytesimpl_1938 {
() => {
// Module: crate::endian_bytes
// Provides: {"impl_1938"}
// Dependencies: {}
impl LintKind { fn allowed (& self , cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { is_lint_allowed (cx , self . as_lint () , expr . hir_id) } fn as_lint (& self) -> & 'static Lint { match self { LintKind :: Host => HOST_ENDIAN_BYTES , LintKind :: Little => LITTLE_ENDIAN_BYTES , LintKind :: Big => BIG_ENDIAN_BYTES , } } fn as_name (& self , prefix : Prefix) -> Symbol { let index = usize :: from (prefix == Prefix :: To) ; match self { LintKind :: Host => HOST_NAMES [index] , LintKind :: Little => LITTLE_NAMES [index] , LintKind :: Big => BIG_NAMES [index] , } } }
};
}
