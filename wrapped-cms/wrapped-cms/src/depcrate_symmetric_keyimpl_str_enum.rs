// Generated macro for impl_str_enum (macro)
macro_rules! Depcrate_symmetric_keyimpl_str_enum {
() => {
// Module: crate::symmetric_key
// Provides: {"impl_str_enum"}
// Dependencies: {}
macro_rules ! impl_str_enum { ($ (# [$ attr : meta]) * $ vis : vis enum $ name : ident { $ ($ (# [$ attr_item : meta]) * $ item : ident => $ value : expr ,) * }) => { $ (# [$ attr]) * $ vis enum $ name { $ ($ (# [$ attr_item]) * $ item ,) * } impl AsRef < str > for $ name { fn as_ref (& self) -> & str { match self { $ (Self ::$ item => $ value ,) * } } } impl <'d > DecodeValue <'d > for $ name { type Error = der :: Error ; fn decode_value < R : Reader <'d >> (reader : & mut R , header : Header) -> der :: Result < Self > { let value = Utf8StringRef :: decode_value (reader , header) ?; match value . as_ref () { $ ($ value => Ok (Self ::$ item) ,) * _ => Err (der :: Error :: new (ErrorKind :: Value { tag : Self :: TAG } , reader . position () ,)) , } } } impl EncodeValue for $ name { fn value_len (& self) -> der :: Result < Length > { Utf8StringRef :: new (self . as_ref ()) ?. value_len () } fn encode_value (& self , encoder : & mut impl Writer) -> der :: Result < () > { Utf8StringRef :: new (self . as_ref ()) ?. encode_value (encoder) } } impl FixedTag for $ name { const TAG : Tag = String :: TAG ; } } ; }
};
}
