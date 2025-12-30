// Generated macro for Field (struct)
macro_rules! Depcrate_dynamic_fieldField {
() => {
// Module: crate::dynamic::field
// Provides: {"Field"}
// Dependencies: {}
# [doc = " A GraphQL field"] pub struct Field { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) arguments : IndexMap < String , InputValue > , pub (crate) ty : TypeRef , pub (crate) ty_str : String , pub (crate) resolver_fn : BoxResolverFn , pub (crate) deprecation : Deprecation , pub (crate) external : bool , pub (crate) requires : Option < String > , pub (crate) provides : Option < String > , pub (crate) shareable : bool , pub (crate) inaccessible : bool , pub (crate) tags : Vec < String > , pub (crate) override_from : Option < String > , pub (crate) directives : Vec < Directive > , pub (crate) requires_scopes : Vec < String > , }
};
}
