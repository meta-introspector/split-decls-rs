// Generated macro for GenericParamSource (enum)
macro_rules! Depcrate_hirGenericParamSource {
() => {
// Module: crate::hir
// Provides: {"GenericParamSource"}
// Dependencies: {}
# [doc = " Records where the generic parameter originated from."] # [doc = ""] # [doc = " This can either be from an item's generics, in which case it's typically"] # [doc = " early-bound (but can be a late-bound lifetime in functions, for example),"] # [doc = " or from a `for<...>` binder, in which case it's late-bound (and notably,"] # [doc = " does not show up in the parent item's generics)."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum GenericParamSource { Generics , Binder , }
};
}
