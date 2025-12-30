// Generated macro for subty_if (function)
macro_rules! Depcrate_utils_tysubty_if {
() => {
// Module: crate::utils::ty
// Provides: {"subty_if"}
// Dependencies: {}
fn subty_if < F > (ty : & Type , f : F) -> Option < & Type > where F : FnOnce (& PathSegment) -> bool , { only_last_segment (ty) . filter (| segment | f (segment)) . and_then (| segment | { if let AngleBracketed (args) = & segment . arguments { only_one (args . args . iter ()) . and_then (| genneric | { if let GenericArgument :: Type (ty) = genneric { Some (ty) } else { None } }) } else { None } }) }
};
}
