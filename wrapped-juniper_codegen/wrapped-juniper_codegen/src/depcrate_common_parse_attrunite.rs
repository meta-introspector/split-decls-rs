// Generated macro for unite (function)
macro_rules! Depcrate_common_parse_attrunite {
() => {
// Module: crate::common::parse::attr
// Provides: {"unite"}
// Dependencies: {}
# [doc = " Prepends the given `attrs` collection with a new [`syn::Attribute`] generated from the given"] # [doc = " `attr_path` and `attr_args`."] # [doc = ""] # [doc = " This function is generally used for uniting `proc_macro_attribute` with its body attributes."] pub (crate) fn unite ((attr_path , attr_args) : (& str , & TokenStream) , attrs : & [syn :: Attribute] ,) -> Vec < syn :: Attribute > { let mut full_attrs = Vec :: with_capacity (attrs . len () + 1) ; let attr_path = syn :: Ident :: new (attr_path , Span :: call_site ()) ; full_attrs . push (parse_quote ! { # [# attr_path (# attr_args)] }) ; full_attrs . extend_from_slice (attrs) ; full_attrs }
};
}
