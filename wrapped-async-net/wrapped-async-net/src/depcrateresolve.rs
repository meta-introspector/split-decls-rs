// Generated macro for resolve (function)
macro_rules! Depcrateresolve {
() => {
// Module: crate
// Provides: {"resolve"}
// Dependencies: {}
# [doc = " Converts or resolves addresses to [`SocketAddr`] values."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " for addr in async_net::resolve(\"google.com:80\").await? {"] # [doc = "     println!(\"{}\", addr);"] # [doc = " }"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn resolve < A : AsyncToSocketAddrs > (addr : A) -> io :: Result < Vec < SocketAddr > > { Ok (addr . to_socket_addrs () . await ? . collect ()) }
};
}
