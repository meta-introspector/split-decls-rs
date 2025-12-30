// Generated macro for pseg_has_single_tyvar (function)
macro_rules! Depcrate_utilpseg_has_single_tyvar {
() => {
// Module: crate::util
// Provides: {"pseg_has_single_tyvar"}
// Dependencies: {}
# [doc = " Returns true iff the given `PathArguments` is one that has one type"] # [doc = " applied to it."] fn pseg_has_single_tyvar (pp : & syn :: PathSegment) -> bool { use syn :: GenericArgument :: Type ; use syn :: PathArguments :: AngleBracketed ; if let AngleBracketed (ab) = & pp . arguments { if let Some (Type (_)) = match_singleton (ab . args . iter ()) { return true ; } } false }
};
}
