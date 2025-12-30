// Generated macro for Block (struct)
macro_rules! Depcrate_hirBlock {
() => {
// Module: crate::hir
// Provides: {"Block"}
// Dependencies: {}
# [doc = " A block of statements `{ .. }`, which may have a label (in this case the"] # [doc = " `targeted_by_break` field will be `true`) and may be `unsafe` by means of"] # [doc = " the `rules` being anything but `DefaultBlock`."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Block < 'hir > { # [doc = " Statements in a block."] pub stmts : & 'hir [Stmt < 'hir >] , # [doc = " An expression at the end of the block"] # [doc = " without a semicolon, if any."] pub expr : Option < & 'hir Expr < 'hir > > , # [stable_hasher (ignore)] pub hir_id : HirId , # [doc = " Distinguishes between `unsafe { ... }` and `{ ... }`."] pub rules : BlockCheckMode , # [doc = " The span includes the curly braces `{` and `}` around the block."] pub span : Span , # [doc = " If true, then there may exist `break 'a` values that aim to"] # [doc = " break out of this block early."] # [doc = " Used by `'label: {}` blocks and by `try {}` blocks."] pub targeted_by_break : bool , }
};
}
