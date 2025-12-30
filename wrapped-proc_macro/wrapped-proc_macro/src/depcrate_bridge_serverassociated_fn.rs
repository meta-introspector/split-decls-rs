// Generated macro for associated_fn (macro)
macro_rules! Depcrate_bridge_serverassociated_fn {
() => {
// Module: crate::bridge::server
// Provides: {"associated_fn"}
// Dependencies: {}
# [doc = " Declare an associated fn of one of the traits below, adding necessary"] # [doc = " default bodies."] macro_rules ! associated_fn { (fn drop (& mut self , $ arg : ident : $ arg_ty : ty)) => (fn drop (& mut self , $ arg : $ arg_ty) { mem :: drop ($ arg) }) ; (fn clone (& mut self , $ arg : ident : $ arg_ty : ty) -> $ ret_ty : ty) => (fn clone (& mut self , $ arg : $ arg_ty) -> $ ret_ty { $ arg . clone () }) ; ($ ($ item : tt) *) => ($ ($ item) *;) }
};
}
