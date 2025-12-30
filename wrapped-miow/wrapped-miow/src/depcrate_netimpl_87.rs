// Generated macro for impl_87 (impl)
macro_rules! Depcrate_netimpl_87 {
() => {
// Module: crate::net
// Provides: {"impl_87"}
// Dependencies: {}
impl TcpStreamExt for TcpStream { unsafe fn read_overlapped (& self , buf : & mut [u8] , overlapped : * mut OVERLAPPED ,) -> io :: Result < Option < usize > > { let buf = slice2buf (buf) ; let mut flags = 0 ; let mut bytes_read : u32 = 0 ; let r = WSARecv (self . as_raw_socket () as SOCKET , & buf , 1 , & mut bytes_read , & mut flags , overlapped , None ,) ; cvt (r , bytes_read) } unsafe fn write_overlapped (& self , buf : & [u8] , overlapped : * mut OVERLAPPED ,) -> io :: Result < Option < usize > > { let buf = slice2buf (buf) ; let mut bytes_written = 0 ; let r = WSASend (self . as_raw_socket () as SOCKET , & buf , 1 , & mut bytes_written , 0 , overlapped , None ,) ; cvt (r , bytes_written) } unsafe fn connect_overlapped (& self , addr : & SocketAddr , buf : & [u8] , overlapped : * mut OVERLAPPED ,) -> io :: Result < Option < usize > > { connect_overlapped (self . as_raw_socket () as SOCKET , addr , buf , overlapped) } fn connect_complete (& self) -> io :: Result < () > { const SO_UPDATE_CONNECT_CONTEXT : i32 = 0x7010 ; let result = unsafe { setsockopt (self . as_raw_socket () as SOCKET , SOL_SOCKET as _ , SO_UPDATE_CONNECT_CONTEXT , std :: ptr :: null_mut () , 0 ,) } ; if result == 0 { Ok (()) } else { Err (io :: Error :: last_os_error ()) } } unsafe fn result (& self , overlapped : * mut OVERLAPPED) -> io :: Result < (usize , u32) > { result (self . as_raw_socket () as SOCKET , overlapped) } }
};
}
