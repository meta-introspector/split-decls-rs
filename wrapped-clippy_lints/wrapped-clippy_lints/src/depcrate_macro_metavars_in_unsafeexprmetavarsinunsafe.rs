// Generated macro for ExprMetavarsInUnsafe (struct)
macro_rules! Depcrate_macro_metavars_in_unsafeExprMetavarsInUnsafe {
() => {
// Module: crate::macro_metavars_in_unsafe
// Provides: {"ExprMetavarsInUnsafe"}
// Dependencies: {}
pub struct ExprMetavarsInUnsafe { warn_unsafe_macro_metavars_in_private_macros : bool , # [doc = " A metavariable can be expanded more than once, potentially across multiple bodies, so it"] # [doc = " requires some state kept across HIR nodes to make it possible to delay a warning"] # [doc = " and later undo:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " macro_rules! x {"] # [doc = "     ($v:expr) => {"] # [doc = "         unsafe { $v; } // unsafe context, it might be possible to emit a warning here, so add it to the map"] # [doc = ""] # [doc = "         $v;            // `$v` expanded another time but in a safe context, set to ReferencedInSafe to suppress"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] metavar_expns : BTreeMap < Span , MetavarState > , }
};
}
