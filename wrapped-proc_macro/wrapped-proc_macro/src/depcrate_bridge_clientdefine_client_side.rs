// Generated macro for define_client_side (macro)
macro_rules! Depcrate_bridge_clientdefine_client_side {
() => {
// Module: crate::bridge::client
// Provides: {"define_client_side"}
// Dependencies: {}
macro_rules ! define_client_side { ($ ($ name : ident { $ (fn $ method : ident ($ ($ arg : ident : $ arg_ty : ty) ,* $ (,) ?) $ (-> $ ret_ty : ty) ?;) * }) ,* $ (,) ?) => { $ (impl $ name { $ (pub (crate) fn $ method ($ ($ arg : $ arg_ty) ,*) $ (-> $ ret_ty) ? { Bridge :: with (| bridge | { let mut buf = bridge . cached_buffer . take () ; buf . clear () ; api_tags :: Method ::$ name (api_tags ::$ name ::$ method) . encode (& mut buf , & mut ()) ; reverse_encode ! (buf ; $ ($ arg) ,*) ; buf = bridge . dispatch . call (buf) ; let r = Result ::< _ , PanicMessage >:: decode (& mut & buf [..] , & mut ()) ; bridge . cached_buffer = buf ; r . unwrap_or_else (| e | panic :: resume_unwind (e . into ())) }) }) * }) * } }
};
}
