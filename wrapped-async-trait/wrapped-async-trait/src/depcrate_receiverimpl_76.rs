// Generated macro for impl_76 (impl)
macro_rules! Depcrate_receiverimpl_76 {
() => {
// Module: crate::receiver
// Provides: {"impl_76"}
// Dependencies: {}
impl VisitMut for HasSelf { fn visit_expr_path_mut (& mut self , expr : & mut ExprPath) { self . 0 |= expr . path . segments [0] . ident == "Self" ; visit_mut :: visit_expr_path_mut (self , expr) ; } fn visit_type_path_mut (& mut self , ty : & mut TypePath) { self . 0 |= ty . path . segments [0] . ident == "Self" ; visit_mut :: visit_type_path_mut (self , ty) ; } fn visit_receiver_mut (& mut self , _arg : & mut Receiver) { self . 0 = true ; } fn visit_item_mut (& mut self , _ : & mut Item) { } fn visit_macro_mut (& mut self , mac : & mut Macro) { if ! contains_fn (mac . tokens . clone ()) { self . 0 |= has_self_in_token_stream (mac . tokens . clone ()) ; } } }
};
}
