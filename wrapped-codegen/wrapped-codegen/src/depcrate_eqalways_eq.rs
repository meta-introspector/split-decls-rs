// Generated macro for always_eq (function)
macro_rules! Depcrate_eqalways_eq {
() => {
// Module: crate::eq
// Provides: {"always_eq"}
// Dependencies: {}
fn always_eq (field_type : & Type) -> bool { match field_type { Type :: Ext (ty) => ty == "Span" , Type :: Token (_) | Type :: Group (_) => true , Type :: Box (inner) => always_eq (inner) , Type :: Tuple (inner) => inner . iter () . all (always_eq) , _ => false , } }
};
}
