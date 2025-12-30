// Generated macro for autoderef_method_receiver (function)
macro_rules! Depcrate_consteval_tests_method_resolutionautoderef_method_receiver {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"autoderef_method_receiver"}
// Dependencies: {}
fn autoderef_method_receiver (table : & mut InferenceTable < '_ > , ty : Ty ,) -> Vec < (Canonical < Ty > , ReceiverAdjustments) > { let mut deref_chain : Vec < _ > = Vec :: new () ; let mut autoderef = autoderef :: Autoderef :: new_no_tracking (table , ty , false , true) ; while let Some ((ty , derefs)) = autoderef . next () { deref_chain . push ((autoderef . table . canonicalize (ty) , ReceiverAdjustments { autoref : None , autoderefs : derefs , unsize_array : false } ,)) ; } if let Some ((TyKind :: Array (parameters , _) , binders , adj)) = deref_chain . last () . map (| (ty , adj) | (ty . value . kind (Interner) , ty . binders . clone () , adj)) { let unsized_ty = TyKind :: Slice (parameters . clone ()) . intern (Interner) ; deref_chain . push ((Canonical { value : unsized_ty , binders } , ReceiverAdjustments { unsize_array : true , .. adj . clone () } ,)) ; } deref_chain }
};
}
