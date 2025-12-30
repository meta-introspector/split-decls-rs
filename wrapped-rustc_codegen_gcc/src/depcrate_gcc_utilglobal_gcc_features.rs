// Generated macro for global_gcc_features (function)
macro_rules! Depcrate_gcc_utilglobal_gcc_features {
() => {
// Module: crate::gcc_util
// Provides: {"global_gcc_features"}
// Dependencies: {}
# [doc = " The list of GCC features computed from CLI flags (`-Ctarget-cpu`, `-Ctarget-feature`,"] # [doc = " `--target` and similar)."] pub (crate) fn global_gcc_features (sess : & Session , diagnostics : bool) -> Vec < String > { let mut features = vec ! [] ; features . extend (sess . target . features . split (',') . filter (| v | ! v . is_empty ()) . map (String :: from)) ; target_features :: flag_to_backend_features (sess , diagnostics , | feature | to_gcc_features (sess , feature) , | feature , enable | { features . extend (to_gcc_features (sess , feature) . iter () . flat_map (| feat | to_gcc_features (sess , feat) . into_iter ()) . map (| feature | { if ! enable { format ! ("-{}" , feature) } else { feature . to_string () } } ,) ,) ; } ,) ; gcc_features_by_flags (sess , & mut features) ; features }
};
}
