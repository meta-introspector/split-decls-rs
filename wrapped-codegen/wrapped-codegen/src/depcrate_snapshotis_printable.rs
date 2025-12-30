// Generated macro for is_printable (function)
macro_rules! Depcrate_snapshotis_printable {
() => {
// Module: crate::snapshot
// Provides: {"is_printable"}
// Dependencies: {}
fn is_printable (ty : & Type) -> bool { match ty { Type :: Ext (name) => name != "Span" , Type :: Box (ty) => is_printable (ty) , Type :: Tuple (ty) => ty . iter () . any (is_printable) , Type :: Token (_) | Type :: Group (_) => false , Type :: Syn (_) | Type :: Std (_) | Type :: Punctuated (_) | Type :: Option (_) | Type :: Vec (_) => true , } }
};
}
