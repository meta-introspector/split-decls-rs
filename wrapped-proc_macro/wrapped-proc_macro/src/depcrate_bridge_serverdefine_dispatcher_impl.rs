// Generated macro for define_dispatcher_impl (macro)
macro_rules! Depcrate_bridge_serverdefine_dispatcher_impl {
() => {
// Module: crate::bridge::server
// Provides: {"define_dispatcher_impl"}
// Dependencies: {}
macro_rules ! define_dispatcher_impl { ($ ($ name : ident { $ (fn $ method : ident ($ ($ arg : ident : $ arg_ty : ty) ,* $ (,) ?) $ (-> $ ret_ty : ty) ?;) * }) ,* $ (,) ?) => { pub trait DispatcherTrait { $ (type $ name ;) * fn dispatch (& mut self , buf : Buffer) -> Buffer ; } impl < S : Server > DispatcherTrait for Dispatcher < MarkedTypes < S >> { $ (type $ name = < MarkedTypes < S > as Types >::$ name ;) * fn dispatch (& mut self , mut buf : Buffer) -> Buffer { let Dispatcher { handle_store , server } = self ; let mut reader = & buf [..] ; match api_tags :: Method :: decode (& mut reader , & mut ()) { $ (api_tags :: Method ::$ name (m) => match m { $ (api_tags ::$ name ::$ method => { let mut call_method = || { reverse_decode ! (reader , handle_store ; $ ($ arg : $ arg_ty) ,*) ; $ name ::$ method (server , $ ($ arg) ,*) } ; let r = if thread :: panicking () { Ok (call_method ()) } else { panic :: catch_unwind (panic :: AssertUnwindSafe (call_method)) . map_err (PanicMessage :: from) } ; buf . clear () ; r . encode (& mut buf , handle_store) ; }) * }) ,* } buf } } } }
};
}
