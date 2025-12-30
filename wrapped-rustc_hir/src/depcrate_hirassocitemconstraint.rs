// Generated macro for AssocItemConstraint (struct)
macro_rules! Depcrate_hirAssocItemConstraint {
() => {
// Module: crate::hir
// Provides: {"AssocItemConstraint"}
// Dependencies: {}
# [doc = " A constraint on an associated item."] # [doc = ""] # [doc = " ### Examples"] # [doc = ""] # [doc = " * the `A = Ty` and `B = Ty` in `Trait<A = Ty, B = Ty>`"] # [doc = " * the `G<Ty> = Ty` in `Trait<G<Ty> = Ty>`"] # [doc = " * the `A: Bound` in `Trait<A: Bound>`"] # [doc = " * the `RetTy` in `Trait(ArgTy, ArgTy) -> RetTy`"] # [doc = " * the `C = { Ct }` in `Trait<C = { Ct }>` (feature `associated_const_equality`)"] # [doc = " * the `f(..): Bound` in `Trait<f(..): Bound>` (feature `return_type_notation`)"] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct AssocItemConstraint < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub ident : Ident , pub gen_args : & 'hir GenericArgs < 'hir > , pub kind : AssocItemConstraintKind < 'hir > , pub span : Span , }
};
}
