// Generated macro for impl_141 (impl)
macro_rules! Depcrateimpl_141 {
() => {
// Module: crate
// Provides: {"impl_141"}
// Dependencies: {}
# [cfg (unix)] impl Async < UnixStream > { # [doc = " Creates a UDS stream connected to the specified path."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use async_io::Async;"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let stream = Async::<UnixStream>::connect(\"/tmp/socket\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn connect < P : AsRef < Path > > (path : P) -> io :: Result < Async < UnixStream > > { let address = convert_path_to_socket_address (path . as_ref ()) ? ; let socket = connect (address . into () , rn :: AddressFamily :: UNIX , None) ? ; let stream = Async :: new_nonblocking (UnixStream :: from (socket)) ? ; stream . writable () . await ? ; stream . get_ref () . peer_addr () ? ; Ok (stream) } # [doc = " Creates an unnamed pair of connected UDS stream sockets."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use async_io::Async;"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let (stream1, stream2) = Async::<UnixStream>::pair()?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub fn pair () -> io :: Result < (Async < UnixStream > , Async < UnixStream >) > { let (stream1 , stream2) = UnixStream :: pair () ? ; Ok ((Async :: new (stream1) ? , Async :: new (stream2) ?)) } }
};
}
