// Generated macro for impl_560 (impl)
macro_rules! Depcrate_lateimpl_560 {
() => {
// Module: crate::late
// Provides: {"impl_560"}
// Dependencies: {}
impl < 'tcx , T : LateLintPass < 'tcx > > LateContextAndPass < 'tcx , T > { # [doc = " Merge the lints specified by any lint attributes into the"] # [doc = " current lint context, call the provided function, then reset the"] # [doc = " lints in effect to their previous state."] fn with_lint_attrs < F > (& mut self , id : HirId , f : F) where F : FnOnce (& mut Self) , { let attrs = self . context . tcx . hir_attrs (id) ; let prev = self . context . last_node_with_lint_attrs ; self . context . last_node_with_lint_attrs = id ; debug ! ("late context: enter_attrs({:?})" , attrs) ; lint_callback ! (self , check_attributes , attrs) ; for attr in attrs { lint_callback ! (self , check_attribute , attr) ; } f (self) ; debug ! ("late context: exit_attrs({:?})" , attrs) ; lint_callback ! (self , check_attributes_post , attrs) ; self . context . last_node_with_lint_attrs = prev ; } fn with_param_env < F > (& mut self , id : hir :: OwnerId , f : F) where F : FnOnce (& mut Self) , { let old_param_env = self . context . param_env ; self . context . param_env = self . context . tcx . param_env (id) ; f (self) ; self . context . param_env = old_param_env ; } fn process_mod (& mut self , m : & 'tcx hir :: Mod < 'tcx > , n : HirId) { lint_callback ! (self , check_mod , m , n) ; hir_visit :: walk_mod (self , m) ; } }
};
}
