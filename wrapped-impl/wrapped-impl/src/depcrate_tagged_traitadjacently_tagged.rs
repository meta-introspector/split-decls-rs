// Generated macro for adjacently_tagged (function)
macro_rules! Depcrate_tagged_traitadjacently_tagged {
() => {
// Module: crate::tagged_trait
// Provides: {"adjacently_tagged"}
// Dependencies: {}
fn adjacently_tagged (tag : LitStr , content : LitStr , default_variant : Option < LitStr > , deny_unknown_fields : bool , input : & ItemTrait ,) -> (TokenStream , TokenStream) { let object = & input . ident ; let object_name = object . to_string () ; let (_ , ty_generics , _) = input . generics . split_for_impl () ; let static_registry = static_registry () ; let default_variant_literal = match default_variant { Some (variant) => quote ! (typetag ::# private :: Option :: Some (# variant)) , None => quote ! (typetag ::# private :: Option :: None) , } ; let serialize_impl = quote ! { let name = < Self as # object # ty_generics >:: typetag_name (self) ; typetag ::# private :: adjacently :: serialize (serializer , # object_name , # tag , name , # content , self) } ; let deserialize_impl = quote ! { # static_registry typetag ::# private :: adjacently :: deserialize (deserializer , # object_name , & [# tag , # content] , # default_variant_literal , registry , # deny_unknown_fields ,) } ; (serialize_impl , deserialize_impl) }
};
}
