// Generated macro for skip (function)
macro_rules! Depcrate_hashskip {
() => {
// Module: crate::hash
// Provides: {"skip"}
// Dependencies: {}
fn skip (field_type : & Type) -> bool { match field_type { Type :: Ext (ty) => ty == "Span" , Type :: Token (_) | Type :: Group (_) => true , Type :: Box (inner) => skip (inner) , Type :: Tuple (inner) => inner . iter () . all (skip) , _ => false , } }
};
}
