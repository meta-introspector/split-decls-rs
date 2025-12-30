// Generated macro for impl_95 (impl)
macro_rules! Depcrate_netimpl_95 {
() => {
// Module: crate::net
// Provides: {"impl_95"}
// Dependencies: {}
impl AcceptAddrsBuf { # [doc = " Creates a new blank buffer ready to be passed to a call to"] # [doc = " `accept_overlapped`."] pub fn new () -> AcceptAddrsBuf { unsafe { mem :: zeroed () } } # [doc = " Parses the data contained in this address buffer, returning the parsed"] # [doc = " result if successful."] # [doc = ""] # [doc = " This function can be called after a call to `accept_overlapped` has"] # [doc = " succeeded to parse out the data that was written in."] pub fn parse (& self , socket : & TcpListener) -> io :: Result < AcceptAddrs < '_ > > { let mut ret = AcceptAddrs { local : std :: ptr :: null_mut () , local_len : 0 , remote : std :: ptr :: null_mut () , remote_len : 0 , _data : self , } ; let ptr = GETACCEPTEXSOCKADDRS . get (socket . as_raw_socket () as SOCKET) ? ; assert ! (ptr != 0) ; unsafe { let get_sockaddrs = mem :: transmute :: < usize , LPFN_GETACCEPTEXSOCKADDRS > (ptr) . unwrap () ; let (a , b , c , d) = self . args () ; get_sockaddrs (a , b , c , d , & mut ret . local , & mut ret . local_len , & mut ret . remote , & mut ret . remote_len ,) ; Ok (ret) } } # [allow (deref_nullptr)] fn args (& self) -> (* mut std :: ffi :: c_void , u32 , u32 , u32) { let remote_offset = mem :: offset_of ! (AcceptAddrsBuf , remote) ; (self as * const _ as * mut _ , 0 , remote_offset as u32 , (mem :: size_of_val (self) - remote_offset) as u32 ,) } }
};
}
