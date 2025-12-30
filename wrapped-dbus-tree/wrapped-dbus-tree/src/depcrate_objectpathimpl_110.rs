// Generated macro for impl_110 (impl)
macro_rules! Depcrate_objectpathimpl_110 {
() => {
// Module: crate::objectpath
// Provides: {"impl_110"}
// Dependencies: {}
impl < M : MethodType < D > , D : DataType > MsgHandler for Tree < M , D > { fn handle_msg (& mut self , msg : & Message) -> Option < MsgHandlerResult > { self . handle (msg) . map (| v | MsgHandlerResult { handled : true , done : false , reply : v }) } fn handler_type (& self) -> MsgHandlerType { MsgHandlerType :: MsgType (MessageType :: MethodCall) } }
};
}
