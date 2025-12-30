// Generated macro for externally_tagged (function)
macro_rules! Depcrate_tagged_traitexternally_tagged {
() => {
// Module: crate::tagged_trait
// Provides: {"externally_tagged"}
// Dependencies: {}
fn externally_tagged (input : & ItemTrait) -> (TokenStream , TokenStream) { let object = & input . ident ; let object_name = object . to_string () ; let (_ , ty_generics , _) = input . generics . split_for_impl () ; let static_registry = static_registry () ; let serialize_impl = quote ! { let name = < Self as # object # ty_generics >:: typetag_name (self) ; typetag ::# private :: externally :: serialize (serializer , name , self) } ; let deserialize_impl = quote ! { # static_registry typetag ::# private :: externally :: deserialize (deserializer , # object_name , registry) } ; (serialize_impl , deserialize_impl) }
};
}
