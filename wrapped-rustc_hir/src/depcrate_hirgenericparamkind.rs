// Generated macro for GenericParamKind (enum)
macro_rules! Depcrate_hirGenericParamKind {
() => {
// Module: crate::hir
// Provides: {"GenericParamKind"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum GenericParamKind < 'hir > { # [doc = " A lifetime definition (e.g., `'a: 'b + 'c + 'd`)."] Lifetime { kind : LifetimeParamKind , } , Type { default : Option < & 'hir Ty < 'hir > > , synthetic : bool , } , Const { ty : & 'hir Ty < 'hir > , # [doc = " Optional default value for the const generic param"] default : Option < & 'hir ConstArg < 'hir > > , } , }
};
}
