// Generated macro for impl_11244 (impl)
macro_rules! Depcrate_unwrap_in_resultimpl_11244 {
() => {
// Module: crate::unwrap_in_result
// Provides: {"impl_11244"}
// Dependencies: {}
impl UnwrapInResult { fn enter_item (& mut self , cx : & LateContext < '_ > , fn_def_id : OwnerId , sig : & FnSig < '_ >) { self . fn_stack . push (self . current_fn . take ()) ; self . current_fn = is_option_or_result (cx , return_ty (cx , fn_def_id)) . map (| kind | OptionOrResultFn { kind , return_ty_span : Some (sig . decl . output . span ()) , }) ; } fn leave_item (& mut self) { self . current_fn = self . fn_stack . pop () . unwrap () ; } }
};
}
