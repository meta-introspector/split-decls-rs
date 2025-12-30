// Generated macro for Declaration (struct)
macro_rules! Depcrate_gather_localsDeclaration {
() => {
// Module: crate::gather_locals
// Provides: {"Declaration"}
// Dependencies: {}
# [doc = " A declaration is an abstraction of [hir::LetStmt] and [hir::LetExpr]."] # [doc = ""] # [doc = " It must have a hir_id, as this is how we connect gather_locals to the check functions."] pub (super) struct Declaration < 'a > { pub hir_id : HirId , pub pat : & 'a hir :: Pat < 'a > , pub ty : Option < & 'a hir :: Ty < 'a > > , pub span : Span , pub init : Option < & 'a hir :: Expr < 'a > > , pub origin : DeclOrigin < 'a > , }
};
}
