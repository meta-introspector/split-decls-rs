// Generated macro for define_cipher_context (macro)
macro_rules! Depcrate_cipherdefine_cipher_context {
() => {
// Module: crate::cipher
// Provides: {"define_cipher_context"}
// Dependencies: {}
macro_rules ! define_cipher_context { ($ name : ident , $ other : ident) => { # [doc = " The contextual data used to encrypt or decrypt data."] # [non_exhaustive] pub enum $ name { # [doc = " A 128-bit Initialization Vector."] Iv128 (FixedLength < IV_LEN_128_BIT >) , # [doc = " No Cipher Context"] None , } impl <'a > TryFrom <&'a $ name > for &'a [u8] { type Error = Unspecified ; fn try_from (value : &'a $ name) -> Result < Self , Unspecified > { match value { $ name :: Iv128 (iv) => Ok (iv . as_ref ()) , _ => Err (Unspecified) , } } } impl Debug for $ name { fn fmt (& self , f : & mut core :: fmt :: Formatter <'_ >) -> core :: fmt :: Result { match self { Self :: Iv128 (_) => write ! (f , "Iv128") , Self :: None => write ! (f , "None") , } } } impl From <$ other > for $ name { fn from (value : $ other) -> Self { match value { $ other :: Iv128 (iv) => $ name :: Iv128 (iv) , $ other :: None => $ name :: None , } } } } ; }
};
}
