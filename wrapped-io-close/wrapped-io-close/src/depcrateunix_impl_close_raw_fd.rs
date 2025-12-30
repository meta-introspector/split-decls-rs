// Generated macro for unix_impl_close_raw_fd (macro)
macro_rules! Depcrateunix_impl_close_raw_fd {
() => {
// Module: crate
// Provides: {"unix_impl_close_raw_fd"}
// Dependencies: {}
macro_rules ! unix_impl_close_raw_fd { ($ ty : ty , "std" $ (,$ lt : lifetime) * $ (,$ id : ident) *) => { unix_impl_close_raw_fd ! ($ ty , "unix" $ (,$ lt) * $ (,$ id) *) ; } ; ($ ty : ty , $ ft_fm : literal $ (,$ lt : lifetime) * $ (,$ id : ident) *) => { # [cfg (unix)] # [cfg (any (feature = $ ft_fm , target_family = $ ft_fm))] # [cfg_attr (all (docsrs , feature = $ ft_fm) , doc (cfg (feature = $ ft_fm)))] impl <$ ($ lt ,) * $ ($ id ,) *> Close for $ ty { # [doc = " Drops an I/O writer containing a raw file descriptor."] fn close (mut self) -> Result < () > { use std :: io :: ErrorKind ; use std :: os :: unix :: io :: IntoRawFd ; self . flush () ?; let fd = self . into_raw_fd () ; let rv = unsafe { libc :: close (fd) } ; if rv != - 1 { Ok (()) } else { match Error :: last_os_error () { e if e . kind () == ErrorKind :: Interrupted => Ok (()) , e => Err (e) , } } } } } ; }
};
}
