// Generated macro for impl_238 (impl)
macro_rules! Depcrate_executor_owned_executorimpl_238 {
() => {
// Module: crate::executor::owned_executor
// Provides: {"impl_238"}
// Dependencies: {}
impl < CtxT , S > Clone for OwnedExecutor < '_ , CtxT , S > where S : Clone , { fn clone (& self) -> Self { Self { fragments : self . fragments . clone () , variables : self . variables . clone () , current_selection_set : self . current_selection_set . clone () , parent_selection_set : self . parent_selection_set . clone () , current_type : self . current_type . clone () , schema : self . schema , context : self . context , errors : RwLock :: new (vec ! []) , field_path : self . field_path . clone () , } } }
};
}
