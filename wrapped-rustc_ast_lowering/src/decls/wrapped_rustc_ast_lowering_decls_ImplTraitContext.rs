use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Context of `impl Trait` in code, which determines whether it is allowed in an HIR subtree,
/// and if so, what meaning it has.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ImplTraitContext {
    /// Treat `impl Trait` as shorthand for a new universal generic parameter.
    /// Example: `fn foo(x: impl Debug)`, where `impl Debug` is conceptually
    /// equivalent to a fresh universal parameter like `fn foo<T: Debug>(x: T)`.
    ///
    /// Newly generated parameters should be inserted into the given `Vec`.
    Universal,
    /// Treat `impl Trait` as shorthand for a new opaque type.
    /// Example: `fn foo() -> impl Debug`, where `impl Debug` is conceptually
    /// equivalent to a new opaque type like `type T = impl Debug; fn foo() -> T`.
    ///
    OpaqueTy {
        origin: hir::OpaqueTyOrigin<LocalDefId>,
    },
    /// Treat `impl Trait` as a "trait ascription", which is like a type
    /// variable but that also enforces that a set of trait goals hold.
    ///
    /// This is useful to guide inference for unnameable types.
    InBinding,
    /// `impl Trait` is unstably accepted in this position.
    FeatureGated(ImplTraitPosition, Symbol),
    /// `impl Trait` is not accepted in this position.
    Disallowed(ImplTraitPosition),
}
