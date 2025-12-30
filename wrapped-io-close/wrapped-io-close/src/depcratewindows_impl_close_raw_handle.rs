// Generated macro for windows_impl_close_raw_handle (macro)
macro_rules! Depcratewindows_impl_close_raw_handle {
() => {
// Module: crate
// Provides: {"windows_impl_close_raw_handle"}
// Dependencies: {}
macro_rules ! windows_impl_close_raw_handle { ($ ty : ty , "std" $ (,$ lt : lifetime) * $ (,$ id : ident) *) => { windows_impl_close_raw_handle ! ($ ty , "windows" $ (,$ lt) * $ (,$ id) *) ; } ; ($ ty : ty , $ ft_fm : literal $ (,$ lt : lifetime) * $ (,$ id : ident) *) => { # [cfg (windows)] # [cfg (any (feature = $ ft_fm , target_family = $ ft_fm))] # [cfg_attr (all (docsrs , feature = $ ft_fm) , doc (cfg (feature = $ ft_fm)))] impl <$ ($ lt ,) * $ ($ id ,) *> Close for $ ty { # [doc = " Drops an I/O writer containing a raw handle."] fn close (mut self) -> Result < () > { use std :: os :: windows :: io :: IntoRawHandle ; use winapi :: um :: handleapi ; self . flush () ?; let handle = self . into_raw_handle () ; let rv = unsafe { handleapi :: CloseHandle (handle) } ; if rv != 0 { Ok (()) } else { Err (Error :: last_os_error ()) } } } } ; }
};
}
