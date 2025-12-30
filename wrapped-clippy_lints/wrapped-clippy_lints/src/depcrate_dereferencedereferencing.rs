// Generated macro for Dereferencing (struct)
macro_rules! Depcrate_dereferenceDereferencing {
() => {
// Module: crate::dereference
// Provides: {"Dereferencing"}
// Dependencies: {}
# [derive (Default)] pub struct Dereferencing < 'tcx > { state : Option < (State , StateData < 'tcx >) > , skip_expr : Option < HirId > , # [doc = " The body the first local was found in. Used to emit lints when the traversal of the body has"] # [doc = " been finished. Note we can't lint at the end of every body as they can be nested within each"] # [doc = " other."] current_body : Option < BodyId > , # [doc = " The list of locals currently being checked by the lint."] # [doc = " If the value is `None`, then the binding has been seen as a ref pattern, but is not linted."] # [doc = " This is needed for or patterns where one of the branches can be linted, but another can not"] # [doc = " be."] # [doc = ""] # [doc = " e.g. `m!(x) | Foo::Bar(ref x)`"] ref_locals : FxIndexMap < HirId , Option < RefPat > > , # [doc = " The outermost `impl Deref` we're currently in. While we're in one,"] # [doc = " `explicit_deref_methods` is deactivated"] outermost_deref_impl : Option < OwnerId > , }
};
}
