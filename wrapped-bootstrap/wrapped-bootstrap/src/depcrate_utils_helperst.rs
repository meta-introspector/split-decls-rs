// Generated macro for t (macro)
macro_rules! Depcrate_utils_helperst {
() => {
// Module: crate::utils::helpers
// Provides: {"t"}
// Dependencies: {}
# [doc = " A helper macro to `unwrap` a result except also print out details like:"] # [doc = ""] # [doc = " * The file/line of the panic"] # [doc = " * The expression that failed"] # [doc = " * The error itself"] # [doc = ""] # [doc = " This is currently used judiciously throughout the build system rather than"] # [doc = " using a `Result` with `try!`, but this may change one day..."] # [macro_export] macro_rules ! t { ($ e : expr) => { { let _panic_guard = $ crate :: PanicTracker (std :: panic :: Location :: caller ()) ; match $ e { Ok (e) => e , Err (e) => panic ! ("{} failed with {}" , stringify ! ($ e) , e) , } } } ; ($ e : expr , $ extra : expr) => { { let _panic_guard = $ crate :: PanicTracker (std :: panic :: Location :: caller ()) ; match $ e { Ok (e) => e , Err (e) => panic ! ("{} failed with {} ({:?})" , stringify ! ($ e) , e , $ extra) , } } } ; }
};
}
