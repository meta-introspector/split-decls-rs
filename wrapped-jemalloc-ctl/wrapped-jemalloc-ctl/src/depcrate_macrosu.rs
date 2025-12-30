// Generated macro for u (macro)
macro_rules! Depcrate_macrosu {
() => {
// Module: crate::macros
// Provides: {"u"}
// Dependencies: {}
# [doc = " Update"] macro_rules ! u { ($ id : ident => $ ret_ty : ty) => { paste :: paste ! { impl $ id { # [doc = " Updates key to `value` returning its old value using string API."] pub fn update (value : $ ret_ty) -> crate :: error :: Result <$ ret_ty > { use crate :: keys :: Access ; Self :: NAME . update (value) } } impl [<$ id _mib >] { # [doc = " Updates key to `value` returning its old value using MIB API."] pub fn update (self , value : $ ret_ty) -> crate :: error :: Result <$ ret_ty > { use crate :: keys :: Access ; self . 0 . update (value) } } # [cfg (test)] # [test] # [allow (unused)] fn [<$ id _update_test >] () { match stringify ! ($ id) { "background_thread" | "max_background_threads" if cfg ! (target_os = "macos") => return , _ => () , } let a = $ id :: update ($ ret_ty :: default ()) . unwrap () ; let mib = $ id :: mib () . unwrap () ; let b = mib . update ($ ret_ty :: default ()) . unwrap () ; # [cfg (feature = "use_std")] println ! (concat ! (stringify ! ($ id) , " (update): (\"{}\", \"{}\") - \"{}\"") , a , b , $ ret_ty :: default ()) ; } } } ; }
};
}
