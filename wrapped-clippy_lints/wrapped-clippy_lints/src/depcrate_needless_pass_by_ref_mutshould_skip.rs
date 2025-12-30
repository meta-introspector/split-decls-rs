// Generated macro for should_skip (function)
macro_rules! Depcrate_needless_pass_by_ref_mutshould_skip {
() => {
// Module: crate::needless_pass_by_ref_mut
// Provides: {"should_skip"}
// Dependencies: {}
fn should_skip < 'tcx > (cx : & LateContext < 'tcx > , input : rustc_hir :: Ty < 'tcx > , ty : Ty < '_ > , arg : & rustc_hir :: Param < '_ > ,) -> bool { if ! matches ! (ty . ref_mutability () , Some (Mutability :: Mut)) { return true ; } if is_self (arg) { return false ; } if let PatKind :: Binding (.. , name , _) = arg . pat . kind && (name . name == kw :: Underscore || name . as_str () . starts_with ('_')) { return true ; } is_from_proc_macro (cx , & input) }
};
}
