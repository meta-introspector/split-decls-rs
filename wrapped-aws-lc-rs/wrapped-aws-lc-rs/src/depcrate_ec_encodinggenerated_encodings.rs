// Generated macro for generated_encodings (macro)
macro_rules! Depcrate_ec_encodinggenerated_encodings {
() => {
// Module: crate::ec::encoding
// Provides: {"generated_encodings"}
// Dependencies: {}
macro_rules ! generated_encodings { ($ (($ name : ident , $ name_type : ident)) ,*) => { use core :: fmt :: { Debug , Error , Formatter } ; use core :: ops :: Deref ; mod buffer_type { $ (pub struct $ name_type { _priv : () , }) * } $ (# [doc = " Serialized bytes"] pub struct $ name <'a > (Buffer <'a , buffer_type ::$ name_type >) ; impl <'a > Deref for $ name <'a > { type Target = Buffer <'a , buffer_type ::$ name_type >; fn deref (& self) -> & Self :: Target { & self . 0 } } impl $ name <'static > { # [allow (dead_code)] pub (crate) fn new (owned : Vec < u8 >) -> Self { Self (Buffer :: new (owned)) } # [allow (dead_code)] pub (crate) fn take_from_slice (owned : & mut [u8]) -> Self { Self (Buffer :: take_from_slice (owned)) } } impl Debug for $ name <'_ > { fn fmt (& self , f : & mut Formatter <'_ >) -> Result < () , Error > { f . debug_struct (stringify ! ($ name)) . finish () } } impl <'a > From < Buffer <'a , buffer_type ::$ name_type >> for $ name <'a > { fn from (value : Buffer <'a , buffer_type ::$ name_type >) -> Self { Self (value) } }) * } }
};
}
