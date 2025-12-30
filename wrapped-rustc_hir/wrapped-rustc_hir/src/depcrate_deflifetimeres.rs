// Generated macro for LifetimeRes (enum)
macro_rules! Depcrate_defLifetimeRes {
() => {
// Module: crate::def
// Provides: {"LifetimeRes"}
// Dependencies: {}
# [doc = " Resolution for a lifetime appearing in a type."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum LifetimeRes { # [doc = " Successfully linked the lifetime to a generic parameter."] Param { # [doc = " Id of the generic parameter that introduced it."] param : LocalDefId , # [doc = " Id of the introducing place. That can be:"] # [doc = " - an item's id, for the item's generic parameters;"] # [doc = " - a TraitRef's ref_id, identifying the `for<...>` binder;"] # [doc = " - a FnPtr type's id."] # [doc = ""] # [doc = " This information is used for impl-trait lifetime captures, to know when to or not to"] # [doc = " capture any given lifetime."] binder : NodeId , } , # [doc = " Created a generic parameter for an anonymous lifetime."] Fresh { # [doc = " Id of the generic parameter that introduced it."] # [doc = ""] # [doc = " Creating the associated `LocalDefId` is the responsibility of lowering."] param : NodeId , # [doc = " Id of the introducing place. See `Param`."] binder : NodeId , # [doc = " Kind of elided lifetime"] kind : hir :: MissingLifetimeKind , } , # [doc = " This variant is used for anonymous lifetimes that we did not resolve during"] # [doc = " late resolution. Those lifetimes will be inferred by typechecking."] Infer , # [doc = " `'static` lifetime."] Static , # [doc = " Resolution failure."] Error , # [doc = " HACK: This is used to recover the NodeId of an elided lifetime."] ElidedAnchor { start : NodeId , end : NodeId } , }
};
}
