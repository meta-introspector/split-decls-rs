// Generated macro for impl_166 (impl)
macro_rules! Depcrate_diagnostic_itemsimpl_166 {
() => {
// Module: crate::diagnostic_items
// Provides: {"impl_166"}
// Dependencies: {}
impl < CTX : crate :: HashStableContext > HashStable < CTX > for DiagnosticItems { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . name_to_id . hash_stable (ctx , hasher) ; } }
};
}
