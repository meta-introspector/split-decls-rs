// Generated macro for impl_492 (impl)
macro_rules! Depcrate_arg_messageitemimpl_492 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_492"}
// Dependencies: {}
impl arg :: RefArg for MessageItem { fn arg_type (& self) -> ArgType { MessageItem :: arg_type (& self) } fn signature (& self) -> Signature < 'static > { MessageItem :: signature (& self) } fn append (& self , i : & mut IterAppend) { arg :: Append :: append_by_ref (self , i) } # [inline] fn as_any (& self) -> & dyn any :: Any where Self : 'static { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any where Self : 'static { self } # [inline] fn box_clone (& self) -> Box < dyn arg :: RefArg + 'static > { Box :: new (self . clone ()) } }
};
}
