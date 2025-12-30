// Generated macro for impl_7833 (impl)
macro_rules! Depcrate_needless_pass_by_ref_mutimpl_7833 {
() => {
// Module: crate::needless_pass_by_ref_mut
// Provides: {"impl_7833"}
// Dependencies: {}
impl MutablyUsedVariablesCtxt < '_ > { fn add_mutably_used_var (& mut self , used_id : HirId) { self . mutably_used_vars . insert (used_id) ; } fn generate_mutably_used_ids_from_aliases (mut self) -> HirIdSet { let all_ids = self . mutably_used_vars . iter () . copied () . collect :: < Vec < _ > > () ; for mut used_id in all_ids { while let Some (id) = self . aliases . get (& used_id) { self . mutably_used_vars . insert (used_id) ; used_id = * id ; } self . mutably_used_vars . insert (used_id) ; } self . mutably_used_vars } fn would_be_alias_cycle (& self , alias : HirId , mut target : HirId) -> bool { while let Some (id) = self . aliases . get (& target) { if * id == alias { return true ; } target = * id ; } false } fn add_alias (& mut self , alias : HirId , target : HirId) { if alias == target || self . would_be_alias_cycle (alias , target) { return ; } self . aliases . insert (alias , target) ; } fn is_in_unsafe_block (& self , item : HirId) -> bool { for (parent , node) in self . tcx . hir_parent_iter (item) { if let Some (fn_sig) = self . tcx . hir_fn_sig_by_hir_id (parent) { return fn_sig . header . is_unsafe () ; } else if let Node :: Block (block) = node && matches ! (block . rules , BlockCheckMode :: UnsafeBlock (_)) { return true ; } } false } }
};
}
