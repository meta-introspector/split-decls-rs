// Generated macro for generic_path_segments (function)
macro_rules! Depcrate_hir_utilsgeneric_path_segments {
() => {
// Module: crate::hir_utils
// Provides: {"generic_path_segments"}
// Dependencies: {}
# [doc = " Returns the segments of a path that might have generic parameters."] # [doc = " Usually just the last segment for free items, except for when the path resolves to an associated"] # [doc = " item, in which case it is the last two"] fn generic_path_segments < 'tcx > (segments : & 'tcx [PathSegment < 'tcx >]) -> Option < & 'tcx [PathSegment < 'tcx >] > { match segments . last () ? . res { Res :: Def (DefKind :: AssocConst | DefKind :: AssocFn | DefKind :: AssocTy , _) => { Some (& segments [segments . len () . checked_sub (2) ? ..]) } , Res :: Err => None , _ => Some (slice :: from_ref (segments . last () ?)) , } }
};
}
