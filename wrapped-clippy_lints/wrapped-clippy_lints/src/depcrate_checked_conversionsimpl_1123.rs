// Generated macro for impl_1123 (impl)
macro_rules! Depcrate_checked_conversionsimpl_1123 {
() => {
// Module: crate::checked_conversions
// Provides: {"impl_1123"}
// Dependencies: {}
impl < 'a > Conversion < 'a > { # [doc = " Combine multiple conversions if the are compatible"] pub fn combine (self , other : Self , cx : & LateContext < '_ >) -> Option < Conversion < 'a > > { if self . is_compatible (& other , cx) { Some (if self . to_type . is_some () { self } else { other }) } else { None } } # [doc = " Checks if two conversions are compatible"] # [doc = " same type of conversion, same 'castee' and same 'to type'"] pub fn is_compatible (& self , other : & Self , cx : & LateContext < '_ >) -> bool { (self . cvt == other . cvt) && (SpanlessEq :: new (cx) . eq_expr (self . expr_to_cast , other . expr_to_cast)) && (self . has_compatible_to_type (other)) } # [doc = " Checks if the to-type is the same (if there is a type constraint)"] fn has_compatible_to_type (& self , other : & Self) -> bool { match (self . to_type , other . to_type) { (Some (l) , Some (r)) => l == r , _ => true , } } # [doc = " Try to construct a new conversion if the conversion type is valid"] fn try_new (expr_to_cast : & 'a Expr < '_ > , from_type : Symbol , to_type : Symbol) -> Option < Conversion < 'a > > { ConversionType :: try_new (from_type , to_type) . map (| cvt | Conversion { cvt , expr_to_cast , to_type : Some (to_type) , }) } # [doc = " Construct a new conversion without type constraint"] fn new_any (expr_to_cast : & 'a Expr < '_ >) -> Conversion < 'a > { Conversion { cvt : ConversionType :: SignedToUnsigned , expr_to_cast , to_type : None , } } }
};
}
