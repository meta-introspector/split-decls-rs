// Generated macro for impl_206 (impl)
macro_rules! Depcrate_diagnostics_find_useimpl_206 {
() => {
// Module: crate::diagnostics::find_use
// Provides: {"impl_206"}
// Dependencies: {}
impl < 'a , 'tcx > Visitor < 'tcx > for DefUseVisitor < 'a , 'tcx > { fn visit_local (& mut self , local : Local , context : PlaceContext , _ : Location) { let local_ty = self . body . local_decls [local] . ty ; let mut found_it = false ; self . tcx . for_each_free_region (& local_ty , | r | { if r . as_var () == self . region_vid { found_it = true ; } }) ; if found_it { self . def_use_result = match def_use :: categorize (context) { Some (DefUse :: Def) => Some (DefUseResult :: Def) , Some (DefUse :: Use) => Some (DefUseResult :: UseLive { local }) , Some (DefUse :: Drop) => Some (DefUseResult :: UseDrop { local }) , None => None , } ; } } }
};
}
