// Generated macro for build_registry (function)
macro_rules! Depcrate_tagged_traitbuild_registry {
() => {
// Module: crate::tagged_trait
// Provides: {"build_registry"}
// Dependencies: {}
fn build_registry (input : & ItemTrait) -> TokenStream { let vis = & input . vis ; let object = & input . ident ; quote ! { type TypetagStrictest = < dyn # object as typetag ::# private :: Strictest >:: Object ; type TypetagFn = typetag ::# private :: DeserializeFn < TypetagStrictest >; # vis struct TypetagRegistration < T > { name : &'static str , deserializer : T , } typetag ::# private :: inventory :: collect ! (TypetagRegistration < TypetagFn >) ; impl dyn # object { # [doc (hidden)] # vis const fn typetag_register < T > (name : &'static str , deserializer : T) -> TypetagRegistration < T > { TypetagRegistration { name , deserializer } } } } }
};
}
