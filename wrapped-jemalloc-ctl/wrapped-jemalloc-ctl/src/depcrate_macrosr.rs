// Generated macro for r (macro)
macro_rules! Depcrate_macrosr {
() => {
// Module: crate::macros
// Provides: {"r"}
// Dependencies: {}
# [doc = " Read"] macro_rules ! r { ($ id : ident => $ ret_ty : ty) => { paste :: paste ! { impl $ id { # [doc = " Reads value using string API."] pub fn read () -> crate :: error :: Result <$ ret_ty > { use crate :: keys :: Access ; Self :: NAME . read () } } impl [<$ id _mib >] { # [doc = " Reads value using MIB API."] pub fn read (self) -> crate :: error :: Result <$ ret_ty > { use crate :: keys :: Access ; self . 0 . read () } } # [cfg (test)] # [test] # [allow (unused)] fn [<$ id _read_test >] () { match stringify ! ($ id) { "background_thread" | "max_background_threads" if cfg ! (target_os = "macos") => return , _ => () , } let a = $ id :: read () . unwrap () ; let mib = $ id :: mib () . unwrap () ; let b = mib . read () . unwrap () ; # [cfg (feature = "use_std")] println ! (concat ! (stringify ! ($ id) , " (read): \"{}\" - \"{}\"") , a , b) ; } } } ; }
};
}
