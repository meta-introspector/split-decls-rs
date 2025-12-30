// Generated macro for impl_90 (impl)
macro_rules! Depcrate_netimpl_90 {
() => {
// Module: crate::net
// Provides: {"impl_90"}
// Dependencies: {}
impl TcpListenerExt for TcpListener { unsafe fn accept_overlapped (& self , socket : & TcpStream , addrs : & mut AcceptAddrsBuf , overlapped : * mut OVERLAPPED ,) -> io :: Result < bool > { static ACCEPTEX : WsaExtension = WsaExtension { guid : GUID { data1 : 0xb5367df1 , data2 : 0xcbac , data3 : 0x11cf , data4 : [0x95 , 0xca , 0x00 , 0x80 , 0x5f , 0x48 , 0xa1 , 0x92] , } , val : AtomicUsize :: new (0) , } ; let ptr = ACCEPTEX . get (self . as_raw_socket () as SOCKET) ? ; assert ! (ptr != 0) ; let accept_ex = mem :: transmute :: < usize , LPFN_ACCEPTEX > (ptr) . unwrap () ; let mut bytes = 0 ; let (a , b , c , d) = (* addrs) . args () ; let r = accept_ex (self . as_raw_socket () as SOCKET , socket . as_raw_socket () as SOCKET , a , b , c , d , & mut bytes , overlapped ,) ; let succeeded = if r == TRUE { true } else { last_err () ? ; false } ; Ok (succeeded) } fn accept_complete (& self , socket : & TcpStream) -> io :: Result < () > { const SO_UPDATE_ACCEPT_CONTEXT : i32 = 0x700B ; let me = self . as_raw_socket () ; let result = unsafe { setsockopt (socket . as_raw_socket () as SOCKET , SOL_SOCKET as _ , SO_UPDATE_ACCEPT_CONTEXT , & me as * const _ as * mut _ , mem :: size_of_val (& me) as i32 ,) } ; if result == 0 { Ok (()) } else { Err (io :: Error :: last_os_error ()) } } unsafe fn result (& self , overlapped : * mut OVERLAPPED) -> io :: Result < (usize , u32) > { result (self . as_raw_socket () as SOCKET , overlapped) } }
};
}
