// Generated macro for errors (macro)
macro_rules! Depcrate_transport_errorerrors {
() => {
// Module: crate::transport_error
// Provides: {"errors"}
// Dependencies: {}
macro_rules ! errors { { $ ($ name : ident ($ val : expr) $ desc : expr ;) * } => { # [allow (non_snake_case , unused)] impl Error { $ (pub (crate) fn $ name < T > (reason : T) -> Self where T : Into < String > { Self { code : Code ::$ name , frame : None , reason : reason . into () , } }) * } impl Code { $ (# [doc = $ desc] pub const $ name : Self = Code ($ val) ;) * } impl fmt :: Debug for Code { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { match self . 0 { $ ($ val => f . write_str (stringify ! ($ name)) ,) * x if (0x100 .. 0x200) . contains (& x) => write ! (f , "Code::crypto({:02x})" , self . 0 as u8) , _ => write ! (f , "Code({:x})" , self . 0) , } } } impl fmt :: Display for Code { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { match self . 0 { $ ($ val => f . write_str ($ desc) ,) * _ if self . 0 >= 0x100 && self . 0 < 0x200 => write ! (f , "the cryptographic handshake failed: error {}" , self . 0 & 0xFF) , _ => f . write_str ("unknown error") , } } } } }
};
}
