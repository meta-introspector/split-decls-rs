// Generated macro for is_no_core_crate (function)
macro_rules! Depcrateis_no_core_crate {
() => {
// Module: crate
// Provides: {"is_no_core_crate"}
// Dependencies: {}
pub fn is_no_core_crate (cx : & LateContext < '_ >) -> bool { find_attr ! (cx . tcx . hir_attrs (hir :: CRATE_HIR_ID) , AttributeKind :: NoCore (..)) }
};
}
