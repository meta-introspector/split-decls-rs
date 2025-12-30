// Generated macro for windows_impl_close_raw_socket (macro)
macro_rules! Depcratewindows_impl_close_raw_socket {
() => {
// Module: crate
// Provides: {"windows_impl_close_raw_socket"}
// Dependencies: {}
macro_rules ! windows_impl_close_raw_socket { ($ ty : ty , "std" $ (,$ lt : lifetime) * $ (,$ id : ident) *) => { windows_impl_close_raw_socket ! ($ ty , "windows" $ (,$ lt) * $ (,$ id) *) ; } ; ($ ty : ty , $ ft_fm : literal $ (,$ lt : lifetime) * $ (,$ id : ident) *) => { # [cfg (windows)] # [cfg (any (feature = $ ft_fm , target_family = $ ft_fm))] # [cfg_attr (all (docsrs , feature = $ ft_fm) , doc (cfg (feature = $ ft_fm)))] impl <$ ($ lt ,) * $ ($ id ,) *> Close for $ ty { # [doc = " Drops an I/O writer containing a raw socket."] fn close (mut self) -> Result < () > { use std :: convert :: TryInto ; use std :: os :: windows :: io :: IntoRawSocket ; use winapi :: um :: winsock2 ; self . flush () ?; let socket = self . into_raw_socket () . try_into () . unwrap () ; let rv = unsafe { winsock2 :: closesocket (socket) } ; if rv == 0 { Ok (()) } else { Err (Error :: from_raw_os_error (unsafe { winsock2 :: WSAGetLastError () })) } } } } ; }
};
}
