// Generated macro for impl_211 (impl)
macro_rules! Depcrate_internals_serializeimpl_211 {
() => {
// Module: crate::internals::serialize
// Provides: {"impl_211"}
// Dependencies: {}
impl FieldId { fn serialize_arg (& self) -> Expr { match self { Self :: Struct (name) => syn :: parse2 (quote ! { & self .# name }) . unwrap () , Self :: StructUnnamed (index) => syn :: parse2 (quote ! { & self .# index }) . unwrap () , Self :: Enum (name) => syn :: parse2 (quote ! { # name }) . unwrap () , Self :: EnumUnnamed (ind) => { let field = Ident :: new (& format ! ("id{}" , ind . index) , Span :: mixed_site ()) ; syn :: parse2 (quote ! { # field }) . unwrap () } } } # [doc = " function which computes derive output [proc_macro2::TokenStream]"] # [doc = " of code, which serializes single field"] pub fn serialize_output (& self , cratename : & Path , serialize_with : Option < ExprPath > ,) -> TokenStream2 { let arg : Expr = self . serialize_arg () ; if let Some (func) = serialize_with { quote ! { # func (# arg , writer) ?; } } else { quote ! { # cratename :: BorshSerialize :: serialize (# arg , writer) ?; } } } pub fn enum_variant_header (& self , skipped : bool) -> Option < TokenStream2 > { match self { Self :: Struct (..) | Self :: StructUnnamed (..) => unreachable ! ("no variant header") , Self :: Enum (name) => (! skipped) . then_some (quote ! { # name , }) , Self :: EnumUnnamed (index) => { let field_ident = if skipped { Ident :: new (& format ! ("_id{}" , index . index) , Span :: mixed_site ()) } else { Ident :: new (& format ! ("id{}" , index . index) , Span :: mixed_site ()) } ; Some (quote ! { # field_ident , }) } } } }
};
}
