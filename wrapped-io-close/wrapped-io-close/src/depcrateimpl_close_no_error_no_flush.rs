// Generated macro for impl_close_no_error_no_flush (macro)
macro_rules! Depcrateimpl_close_no_error_no_flush {
() => {
// Module: crate
// Provides: {"impl_close_no_error_no_flush"}
// Dependencies: {}
macro_rules ! impl_close_no_error_no_flush { ($ ty : ty , "std" $ (,$ lt : lifetime) * $ (,$ id : ident) *) => { # [cfg (unix)] impl_close_no_error_no_flush ! ($ ty , "unix" $ (,$ lt) * $ (,$ id) *) ; # [cfg (windows)] impl_close_no_error_no_flush ! ($ ty , "windows" $ (,$ lt) * $ (,$ id) *) ; } ; ($ ty : ty , $ ft_fm : literal $ (,$ lt : lifetime) * $ (,$ id : ident) *) => { # [cfg (any (unix , windows))] # [cfg (any (feature = $ ft_fm , target_family = $ ft_fm))] # [cfg_attr (all (docsrs , feature = $ ft_fm) , doc (cfg (feature = $ ft_fm)))] impl <$ ($ lt ,) * $ ($ id ,) *> Close for $ ty { # [doc = " Drops an I/O writer for which `close()` never produces"] # [doc = " an error, and for which flushing is unnecessary."] # [inline] fn close (self) -> Result < () > { Ok (()) } } } ; }
};
}
