// Generated macro for deref_ty_if_possible (function)
macro_rules! Depcrate_opderef_ty_if_possible {
() => {
// Module: crate::op
// Provides: {"deref_ty_if_possible"}
// Dependencies: {}
# [doc = " Dereferences a single level of immutable referencing."] fn deref_ty_if_possible (ty : Ty < '_ >) -> Ty < '_ > { match ty . kind () { ty :: Ref (_ , ty , hir :: Mutability :: Not) => * ty , _ => ty , } }
};
}
