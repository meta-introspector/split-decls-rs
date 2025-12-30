// Generated macro for qpath_certainty (function)
macro_rules! Depcrate_ty_type_certaintyqpath_certainty {
() => {
// Module: crate::ty::type_certainty
// Provides: {"qpath_certainty"}
// Dependencies: {}
# [doc = " Tries to tell whether a `QPath` resolves to something certain, e.g., whether all of its path"] # [doc = " segments generic arguments are instantiated."] # [doc = ""] # [doc = " `qpath` could refer to either a type or a value. The heuristic never needs the `DefId` of a"] # [doc = " value. So `DefId`s are retained only when `resolves_to_type` is true."] fn qpath_certainty (cx : & LateContext < '_ > , qpath : & QPath < '_ > , resolves_to_type : bool) -> Certainty { let certainty = match qpath { QPath :: Resolved (ty , path) => { let len = path . segments . len () ; path . segments . iter () . enumerate () . fold (ty . map_or (Certainty :: Uncertain , | ty | type_certainty (cx , ty)) , | parent_certainty , (i , path_segment) | { path_segment_certainty (cx , parent_certainty , path_segment , i != len - 1 || resolves_to_type) } ,) } , QPath :: TypeRelative (ty , path_segment) => { path_segment_certainty (cx , type_certainty (cx , ty) , path_segment , resolves_to_type) } , } ; debug_assert ! (resolves_to_type || certainty . to_def_id () . is_none ()) ; certainty }
};
}
