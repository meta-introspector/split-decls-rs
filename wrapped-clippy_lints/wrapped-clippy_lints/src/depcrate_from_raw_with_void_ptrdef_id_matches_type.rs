// Generated macro for def_id_matches_type (function)
macro_rules! Depcrate_from_raw_with_void_ptrdef_id_matches_type {
() => {
// Module: crate::from_raw_with_void_ptr
// Provides: {"def_id_matches_type"}
// Dependencies: {}
# [doc = " Checks whether a `DefId` matches `Box`, `Rc`, `Arc`, or one of the `Weak` types."] # [doc = " Returns a static string slice with the name of the type, if one was found."] fn def_id_matches_type (cx : & LateContext < '_ > , def_id : DefId) -> Option < & 'static str > { if Some (def_id) == cx . tcx . lang_items () . owned_box () { return Some ("Box") ; } if let Some (symbol) = cx . tcx . get_diagnostic_name (def_id) { if symbol == sym :: Arc { return Some ("Arc") ; } else if symbol == sym :: Rc { return Some ("Rc") ; } } if matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: RcWeak | sym :: ArcWeak)) { Some ("Weak") } else { None } }
};
}
