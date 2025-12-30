// Generated macro for encode_args_impl (macro)
macro_rules! Depcrate_encodeencode_args_impl {
() => {
// Module: crate::encode
// Provides: {"encode_args_impl"}
// Dependencies: {}
macro_rules ! encode_args_impl { ($ ($ a : ident : $ T : ident) ,*) => { impl <$ ($ T : EncodeArgument) ,*> args_private :: Sealed for ($ ($ T ,) *) { } impl <$ ($ T : EncodeArgument) ,*> EncodeArguments for ($ ($ T ,) *) { const ENCODINGS : &'static [Encoding] = & [$ (decay_parameter_encoding ($ T :: ENCODING_ARGUMENT)) ,*] ; # [inline] unsafe fn __invoke < R : EncodeReturn > (msg_send_fn : Imp , receiver : * mut AnyObject , sel : Sel , ($ ($ a ,) *) : Self) -> R { let msg_send_fn : unsafe extern "C-unwind" fn (* mut AnyObject , Sel $ (, $ T) *) -> R = unsafe { mem :: transmute (msg_send_fn) } ; unsafe { msg_send_fn (receiver , sel $ (, $ a) *) } } } } ; }
};
}
