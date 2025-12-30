// Generated macro for is_likely_bounds_affecting (function)
macro_rules! Depcrate_name_translationis_likely_bounds_affecting {
() => {
// Module: crate::name_translation
// Provides: {"is_likely_bounds_affecting"}
// Dependencies: {}
# [doc = " Whether a parameter or function name is likely to require a bounds check."] # [doc = ""] # [doc = " This is only a best-effort heuristic, the library author may have called"] # [doc = " this any number of other things."] pub (crate) fn is_likely_bounds_affecting (name : & str) -> bool { let name = name . to_lowercase () ; name . contains ("idx") || name . contains ("index") || name == "i" || name . contains ("capacity") || name . contains ("range") || name . contains ("offset") || name . contains ("count") || name . contains ("stride") || name . contains ("size") }
};
}
