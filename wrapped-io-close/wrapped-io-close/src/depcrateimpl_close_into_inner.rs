// Generated macro for impl_close_into_inner (macro)
macro_rules! Depcrateimpl_close_into_inner {
() => {
// Module: crate
// Provides: {"impl_close_into_inner"}
// Dependencies: {}
macro_rules ! impl_close_into_inner { ($ ty : ty , "std" $ (,$ lt : lifetime) * $ (,$ id : ident) *) => { # [cfg (unix)] impl_close_into_inner ! ($ ty , "unix" $ (,$ lt) * $ (,$ id) *) ; # [cfg (windows)] impl_close_into_inner ! ($ ty , "windows" $ (,$ lt) * $ (,$ id) *) ; } ; ($ ty : ty , $ ft_fm : literal $ (,$ lt : lifetime) * $ (,$ id : ident) *) => { # [cfg (any (unix , windows))] # [cfg (any (feature = $ ft_fm , target_family = $ ft_fm))] # [cfg_attr (all (docsrs , feature = $ ft_fm) , doc (cfg (feature = $ ft_fm)))] impl <$ ($ lt ,) * W : Close , $ ($ id ,) *> Close for $ ty { # [doc = " Drops an I/O writer which can be unwrapped using"] # [doc = " `into_inner()` to return an underlying writer."] fn close (self) -> Result < () > { self . into_inner () ?. close () } } } ; }
};
}
