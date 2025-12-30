// Generated macro for connect_overlapped (function)
macro_rules! Depcrate_netconnect_overlapped {
() => {
// Module: crate::net
// Provides: {"connect_overlapped"}
// Dependencies: {}
unsafe fn connect_overlapped (socket : SOCKET , addr : & SocketAddr , buf : & [u8] , overlapped : * mut OVERLAPPED ,) -> io :: Result < Option < usize > > { static CONNECTEX : WsaExtension = WsaExtension { guid : GUID { data1 : 0x25a207b9 , data2 : 0xddf3 , data3 : 0x4660 , data4 : [0x8e , 0xe9 , 0x76 , 0xe5 , 0x8c , 0x74 , 0x06 , 0x3e] , } , val : AtomicUsize :: new (0) , } ; let ptr = CONNECTEX . get (socket) ? ; assert ! (ptr != 0) ; let connect_ex = mem :: transmute :: < usize , LPFN_CONNECTEX > (ptr) . unwrap () ; let (addr_buf , addr_len) = socket_addr_to_ptrs (addr) ; let mut bytes_sent : u32 = 0 ; let r = connect_ex (socket , addr_buf . as_ptr () , addr_len , buf . as_ptr () as * mut _ , buf . len () as u32 , & mut bytes_sent , overlapped ,) ; if r == TRUE { Ok (Some (bytes_sent as usize)) } else { last_err () } }
};
}
