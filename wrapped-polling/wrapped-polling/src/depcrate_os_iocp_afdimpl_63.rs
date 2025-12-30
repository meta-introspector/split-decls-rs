// Generated macro for impl_63 (impl)
macro_rules! Depcrate_os_iocp_afdimpl_63 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"impl_63"}
// Dependencies: {}
impl fmt :: Debug for AfdPollMask { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { const FLAGS : & [(& str , AfdPollMask)] = & [("RECEIVE" , AfdPollMask :: RECEIVE) , ("RECEIVE_EXPEDITED" , AfdPollMask :: RECEIVE_EXPEDITED) , ("SEND" , AfdPollMask :: SEND) , ("DISCONNECT" , AfdPollMask :: DISCONNECT) , ("ABORT" , AfdPollMask :: ABORT) , ("LOCAL_CLOSE" , AfdPollMask :: LOCAL_CLOSE) , ("ACCEPT" , AfdPollMask :: ACCEPT) , ("CONNECT_FAIL" , AfdPollMask :: CONNECT_FAIL) ,] ; let mut first = true ; for (name , value) in FLAGS { if self . intersects (* value) { if ! first { write ! (f , " | ") ? ; } first = false ; write ! (f , "{name}") ? ; } } Ok (()) } }
};
}
