// Generated macro for pointer_ty (function)
macro_rules! Depcrate_commonpointer_ty {
() => {
// Module: crate::common
// Provides: {"pointer_ty"}
// Dependencies: {}
pub (crate) fn pointer_ty (tcx : TyCtxt < '_ >) -> types :: Type { match tcx . data_layout . pointer_size () . bits () { 16 => types :: I16 , 32 => types :: I32 , 64 => types :: I64 , bits => bug ! ("ptr_sized_integer: unknown pointer bit size {}" , bits) , } }
};
}
