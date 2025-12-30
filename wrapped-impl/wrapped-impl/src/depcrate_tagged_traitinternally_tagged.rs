// Generated macro for internally_tagged (function)
macro_rules! Depcrate_tagged_traitinternally_tagged {
() => {
// Module: crate::tagged_trait
// Provides: {"internally_tagged"}
// Dependencies: {}
fn internally_tagged (tag : LitStr , default_variant : Option < LitStr > , input : & ItemTrait ,) -> (TokenStream , TokenStream) { let object = & input . ident ; let object_name = object . to_string () ; let (_ , ty_generics , _) = input . generics . split_for_impl () ; let static_registry = static_registry () ; let default_variant_literal = match default_variant { Some (variant) => quote ! (typetag ::# private :: Option :: Some (# variant)) , None => quote ! (typetag ::# private :: Option :: None) , } ; let serialize_impl = quote ! { let name = < Self as # object # ty_generics >:: typetag_name (self) ; typetag ::# private :: internally :: serialize (serializer , # tag , name , self) } ; let deserialize_impl = quote ! { # static_registry typetag ::# private :: internally :: deserialize (deserializer , # object_name , # tag , # default_variant_literal , registry) } ; (serialize_impl , deserialize_impl) }
};
}
