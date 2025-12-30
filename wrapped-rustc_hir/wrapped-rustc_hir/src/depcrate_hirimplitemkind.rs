// Generated macro for ImplItemKind (enum)
macro_rules! Depcrate_hirImplItemKind {
() => {
// Module: crate::hir
// Provides: {"ImplItemKind"}
// Dependencies: {}
# [doc = " Represents various kinds of content within an `impl`."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum ImplItemKind < 'hir > { # [doc = " An associated constant of the given type, set to the constant result"] # [doc = " of the expression."] Const (& 'hir Ty < 'hir > , BodyId) , # [doc = " An associated function implementation with the given signature and body."] Fn (FnSig < 'hir > , BodyId) , # [doc = " An associated type."] Type (& 'hir Ty < 'hir >) , }
};
}
