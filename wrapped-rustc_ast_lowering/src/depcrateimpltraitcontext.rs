// Generated macro for ImplTraitContext (enum)
macro_rules! DepcrateImplTraitContext {
() => {
// Module: crate
// Provides: {"ImplTraitContext"}
// Dependencies: {}
# [doc = " Context of `impl Trait` in code, which determines whether it is allowed in an HIR subtree,"] # [doc = " and if so, what meaning it has."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] enum ImplTraitContext { # [doc = " Treat `impl Trait` as shorthand for a new universal generic parameter."] # [doc = " Example: `fn foo(x: impl Debug)`, where `impl Debug` is conceptually"] # [doc = " equivalent to a fresh universal parameter like `fn foo<T: Debug>(x: T)`."] # [doc = ""] # [doc = " Newly generated parameters should be inserted into the given `Vec`."] Universal , # [doc = " Treat `impl Trait` as shorthand for a new opaque type."] # [doc = " Example: `fn foo() -> impl Debug`, where `impl Debug` is conceptually"] # [doc = " equivalent to a new opaque type like `type T = impl Debug; fn foo() -> T`."] # [doc = ""] OpaqueTy { origin : hir :: OpaqueTyOrigin < LocalDefId > } , # [doc = " Treat `impl Trait` as a \"trait ascription\", which is like a type"] # [doc = " variable but that also enforces that a set of trait goals hold."] # [doc = ""] # [doc = " This is useful to guide inference for unnameable types."] InBinding , # [doc = " `impl Trait` is unstably accepted in this position."] FeatureGated (ImplTraitPosition , Symbol) , # [doc = " `impl Trait` is not accepted in this position."] Disallowed (ImplTraitPosition) , }
};
}
