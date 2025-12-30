// Generated macro for impl_602 (impl)
macro_rules! Depcrate_dynamic_fieldimpl_602 {
() => {
// Module: crate::dynamic::field
// Provides: {"impl_602"}
// Dependencies: {}
impl Field { # [doc = " Create a GraphQL field"] pub fn new < N , T , F > (name : N , ty : T , resolver_fn : F) -> Self where N : Into < String > , T : Into < TypeRef > , F : for < 'a > Fn (ResolverContext < 'a >) -> FieldFuture < 'a > + Send + Sync + 'static , { let ty = ty . into () ; Self { name : name . into () , description : None , arguments : Default :: default () , ty_str : ty . to_string () , ty , resolver_fn : Box :: new (resolver_fn) , deprecation : Deprecation :: NoDeprecated , external : false , requires : None , provides : None , shareable : false , inaccessible : false , tags : Vec :: new () , override_from : None , directives : Vec :: new () , requires_scopes : Vec :: new () , } } impl_set_description ! () ; impl_set_deprecation ! () ; impl_set_external ! () ; impl_set_requires ! () ; impl_set_provides ! () ; impl_set_shareable ! () ; impl_set_inaccessible ! () ; impl_set_tags ! () ; impl_set_override_from ! () ; impl_directive ! () ; # [doc = " Add an argument to the field"] # [inline] pub fn argument (mut self , input_value : InputValue) -> Self { self . arguments . insert (input_value . name . clone () , input_value) ; self } }
};
}
