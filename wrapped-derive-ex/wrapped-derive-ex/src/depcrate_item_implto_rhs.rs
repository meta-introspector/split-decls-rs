// Generated macro for to_rhs (function)
macro_rules! Depcrate_item_implto_rhs {
() => {
// Module: crate::item_impl
// Provides: {"to_rhs"}
// Dependencies: {}
fn to_rhs (s : & PathSegment , self_ty : & Type) -> Type { if let PathArguments :: AngleBracketed (args) = & s . arguments { if args . args . len () == 1 { if let GenericArgument :: Type (ty) = & args . args [0] { return expand_self (ty , self_ty) ; } } } self_ty . clone () }
};
}
