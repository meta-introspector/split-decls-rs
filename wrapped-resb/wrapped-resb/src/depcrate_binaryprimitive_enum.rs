// Generated macro for primitive_enum (macro)
macro_rules! Depcrate_binaryprimitive_enum {
() => {
// Module: crate::binary
// Provides: {"primitive_enum"}
// Dependencies: {}
# [doc = " Adds convenience properties to an `enum` represented as a primitive type,"] # [doc = " including conversions to and from the primitive type."] macro_rules ! primitive_enum { ($ type : ty , $ (# [$ meta : meta]) * $ vis : vis enum $ name : ident { $ ($ (# [$ variant_meta : meta]) * $ variant : ident = $ value : expr ,) * }) => { $ (# [$ meta]) * # [repr ($ type)] $ vis enum $ name { $ ($ (# [$ variant_meta]) * $ variant = $ value ,) * } impl From <$ name > for $ type { fn from (v : $ name) -> Self { v as $ type } } impl TryFrom <$ type > for $ name { type Error = BinaryDeserializerError ; fn try_from (value : $ type) -> Result < Self , Self :: Error > { match value { $ (x if x == $ name ::$ variant as $ type => Ok ($ name ::$ variant) ,) * _ => Err (BinaryDeserializerError :: invalid_data (concat ! ("unrecognized value for " , stringify ! ($ name)))) , } } } } }
};
}
