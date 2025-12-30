// Generated macro for impl_29 (impl)
macro_rules! Depcrate_ffdheimpl_29 {
() => {
// Module: crate::ffdhe
// Provides: {"impl_29"}
// Dependencies: {}
impl ActiveKeyExchange for ActiveFfdheKx { fn complete (self : Box < Self > , peer_pub_key : & [u8]) -> Result < SharedSecret , rustls :: Error > { let peer_pub = BigUint :: from_bytes_be (peer_pub_key) ; let secret = peer_pub . modpow (& self . x , & self . p) ; let secret = to_bytes_be_with_len (secret , self . group . p . len ()) ; Ok (SharedSecret :: from (& secret [..])) } fn pub_key (& self) -> & [u8] { & self . x_pub } fn ffdhe_group (& self) -> Option < FfdheGroup < 'static > > { Some (self . group) } fn group (& self) -> NamedGroup { self . named_group } }
};
}
