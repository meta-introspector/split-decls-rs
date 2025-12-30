// Generated macro for define_mark_types_impls (macro)
macro_rules! Depcrate_bridge_serverdefine_mark_types_impls {
() => {
// Module: crate::bridge::server
// Provides: {"define_mark_types_impls"}
// Dependencies: {}
macro_rules ! define_mark_types_impls { ($ ($ name : ident { $ (fn $ method : ident ($ ($ arg : ident : $ arg_ty : ty) ,* $ (,) ?) $ (-> $ ret_ty : ty) ?;) * }) ,* $ (,) ?) => { impl < S : Types > Types for MarkedTypes < S > { $ (type $ name = Marked < S ::$ name , client ::$ name >;) * } $ (impl < S : $ name > $ name for MarkedTypes < S > { $ (fn $ method (& mut self , $ ($ arg : $ arg_ty) ,*) $ (-> $ ret_ty) ? { < _ >:: mark ($ name ::$ method (& mut self . 0 , $ ($ arg . unmark ()) ,*)) }) * }) * } }
};
}
