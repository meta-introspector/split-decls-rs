// Generated macro for impl_intern_key (macro)
macro_rules! Depcrateimpl_intern_key {
() => {
// Module: crate
// Provides: {"impl_intern_key"}
// Dependencies: {}
# [macro_export] macro_rules ! impl_intern_key { ($ id : ident , $ loc : ident) => { # [salsa_macros :: interned (no_lifetime , revisions = usize :: MAX)] # [derive (PartialOrd , Ord)] pub struct $ id { pub loc : $ loc , } impl :: std :: fmt :: Debug for $ id { fn fmt (& self , f : & mut :: std :: fmt :: Formatter <'_ >) -> :: std :: fmt :: Result { f . debug_tuple (stringify ! ($ id)) . field (& format_args ! ("{:04x}" , self . 0 . index ())) . finish () } } } ; }
};
}
