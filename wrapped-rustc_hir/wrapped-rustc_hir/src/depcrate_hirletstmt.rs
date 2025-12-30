// Generated macro for LetStmt (struct)
macro_rules! Depcrate_hirLetStmt {
() => {
// Module: crate::hir
// Provides: {"LetStmt"}
// Dependencies: {}
# [doc = " Represents a `let` statement (i.e., `let <pat>:<ty> = <init>;`)."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct LetStmt < 'hir > { # [doc = " Span of `super` in `super let`."] pub super_ : Option < Span > , pub pat : & 'hir Pat < 'hir > , # [doc = " Type annotation, if any (otherwise the type will be inferred)."] pub ty : Option < & 'hir Ty < 'hir > > , # [doc = " Initializer expression to set the value, if any."] pub init : Option < & 'hir Expr < 'hir > > , # [doc = " Else block for a `let...else` binding."] pub els : Option < & 'hir Block < 'hir > > , # [stable_hasher (ignore)] pub hir_id : HirId , pub span : Span , # [doc = " Can be `ForLoopDesugar` if the `let` statement is part of a `for` loop"] # [doc = " desugaring, or `AssignDesugar` if it is the result of a complex"] # [doc = " assignment desugaring. Otherwise will be `Normal`."] pub source : LocalSource , }
};
}
