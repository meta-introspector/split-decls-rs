// Generated macro for declare_server_traits (macro)
macro_rules! Depcrate_bridge_serverdeclare_server_traits {
() => {
// Module: crate::bridge::server
// Provides: {"declare_server_traits"}
// Dependencies: {}
macro_rules ! declare_server_traits { ($ ($ name : ident { $ (fn $ method : ident ($ ($ arg : ident : $ arg_ty : ty) ,* $ (,) ?) $ (-> $ ret_ty : ty) ?;) * }) ,* $ (,) ?) => { $ (pub trait $ name : Types { $ (associated_fn ! (fn $ method (& mut self , $ ($ arg : $ arg_ty) ,*) $ (-> $ ret_ty) ?) ;) * }) * pub trait Server : Types $ (+ $ name) * { fn globals (& mut self) -> ExpnGlobals < Self :: Span >; # [doc = " Intern a symbol received from RPC"] fn intern_symbol (ident : & str) -> Self :: Symbol ; # [doc = " Recover the string value of a symbol, and invoke a callback with it."] fn with_symbol_string (symbol : & Self :: Symbol , f : impl FnOnce (& str)) ; } } }
};
}
