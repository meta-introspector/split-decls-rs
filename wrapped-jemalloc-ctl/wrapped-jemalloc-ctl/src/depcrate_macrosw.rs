// Generated macro for w (macro)
macro_rules! Depcrate_macrosw {
() => {
// Module: crate::macros
// Provides: {"w"}
// Dependencies: {}
# [doc = " Write"] macro_rules ! w { ($ id : ident => $ ret_ty : ty) => { paste :: paste ! { impl $ id { # [doc = " Writes `value` using string API."] pub fn write (value : $ ret_ty) -> crate :: error :: Result < () > { use crate :: keys :: Access ; Self :: NAME . write (value) } } impl [<$ id _mib >] { # [doc = " Writes `value` using MIB API."] pub fn write (self , value : $ ret_ty) -> crate :: error :: Result < () > { use crate :: keys :: Access ; self . 0 . write (value) } } # [cfg (test)] # [test] fn [<$ id _write_test >] () { match stringify ! ($ id) { "background_thread" | "max_background_threads" if cfg ! (target_os = "macos") => return , _ => () , } let _ = $ id :: write ($ ret_ty :: default ()) . unwrap () ; let mib = $ id :: mib () . unwrap () ; let _ = mib . write ($ ret_ty :: default ()) . unwrap () ; # [cfg (feature = "use_std")] println ! (concat ! (stringify ! ($ id) , " (write): \"{}\"") , $ ret_ty :: default ()) ; } } } ; }
};
}
