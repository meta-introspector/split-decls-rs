// Generated macro for LifetimeKind (enum)
macro_rules! Depcrate_hirLifetimeKind {
() => {
// Module: crate::hir
// Provides: {"LifetimeKind"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq , Hash , HashStable_Generic)] pub enum LifetimeKind { # [doc = " User-given names or fresh (synthetic) names."] Param (LocalDefId) , # [doc = " Implicit lifetime in a context like `dyn Foo`. This is"] # [doc = " distinguished from implicit lifetimes elsewhere because the"] # [doc = " lifetime that they default to must appear elsewhere within the"] # [doc = " enclosing type. This means that, in an `impl Trait` context, we"] # [doc = " don't have to create a parameter for them. That is, `impl"] # [doc = " Trait<Item = &u32>` expands to an opaque type like `type"] # [doc = " Foo<'a> = impl Trait<Item = &'a u32>`, but `impl Trait<item ="] # [doc = " dyn Bar>` expands to `type Foo = impl Trait<Item = dyn Bar +"] # [doc = " 'static>`. The latter uses `ImplicitObjectLifetimeDefault` so"] # [doc = " that surrounding code knows not to create a lifetime"] # [doc = " parameter."] ImplicitObjectLifetimeDefault , # [doc = " Indicates an error during lowering (usually `'_` in wrong place)"] # [doc = " that was already reported."] Error , # [doc = " User wrote an anonymous lifetime, either `'_` or nothing (which gets"] # [doc = " converted to `'_`). The semantics of this lifetime should be inferred"] # [doc = " by typechecking code."] Infer , # [doc = " User wrote `'static` or nothing (which gets converted to `'_`)."] Static , }
};
}
