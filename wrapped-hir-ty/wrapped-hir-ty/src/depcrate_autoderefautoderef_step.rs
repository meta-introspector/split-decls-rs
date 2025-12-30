// Generated macro for autoderef_step (function)
macro_rules! Depcrate_autoderefautoderef_step {
() => {
// Module: crate::autoderef
// Provides: {"autoderef_step"}
// Dependencies: {}
pub (crate) fn autoderef_step (table : & mut InferenceTable < '_ > , ty : Ty , explicit : bool , use_receiver_trait : bool ,) -> Option < (AutoderefKind , Ty) > { if let Some (derefed) = builtin_deref (table . db , & ty , explicit) { Some ((AutoderefKind :: Builtin , table . resolve_ty_shallow (derefed))) } else { Some ((AutoderefKind :: Overloaded , deref_by_trait (table , ty , use_receiver_trait) ?)) } }
};
}
