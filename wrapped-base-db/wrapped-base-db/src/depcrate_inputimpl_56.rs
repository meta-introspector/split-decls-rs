// Generated macro for impl_56 (impl)
macro_rules! Depcrate_inputimpl_56 {
() => {
// Module: crate::input
// Provides: {"impl_56"}
// Dependencies: {}
impl CrateDisplayName { pub fn from_canonical_name (canonical_name : & str) -> CrateDisplayName { let crate_name = CrateName :: normalize_dashes (canonical_name) ; CrateDisplayName { crate_name , canonical_name : Symbol :: intern (canonical_name) } } }
};
}
