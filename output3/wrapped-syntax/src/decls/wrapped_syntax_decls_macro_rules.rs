use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Matches a `SyntaxNode` against an `ast` type.
///
/// # Example:
///
/// ```ignore
/// match_ast! {
///     match node {
///         ast::CallExpr(it) => { ... },
///         ast::MethodCallExpr(it) => { ... },
///         ast::MacroCall(it) => { ... },
///         _ => None,
///     }
/// }
/// ```
#[macro_export]
macro_rules! match_ast {
    (match $node:ident { $($tt:tt)* }) => {
        $crate::match_ast!(match ($node) { $($tt)* })
    };
    (
        match ($node:expr) { $($($path:ident)::+ ($it:pat) => $res:expr,)* _ =>
        $catch_all:expr $(,)? }
    ) => {
        { $(if let Some($it) = $($path ::)+ cast($node .clone()) { $res } else)* {
        $catch_all } }
    };
}
