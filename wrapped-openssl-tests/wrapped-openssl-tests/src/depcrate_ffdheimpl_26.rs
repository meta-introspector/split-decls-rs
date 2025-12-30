// Generated macro for impl_26 (impl)
macro_rules! Depcrate_ffdheimpl_26 {
() => {
// Module: crate::ffdhe
// Provides: {"impl_26"}
// Dependencies: {}
impl SupportedKxGroup for FfdheKxGroup { fn start (& self) -> Result < StartedKeyExchange , rustls :: Error > { let mut x = vec ! [0 ; 64] ; provider :: DEFAULT_PROVIDER . secure_random . fill (& mut x) ? ; let x = BigUint :: from_bytes_be (& x) ; let group = self . 1 ; let p = BigUint :: from_bytes_be (group . p) ; let g = BigUint :: from_bytes_be (group . g) ; let x_pub = g . modpow (& x , & p) ; let x_pub = to_bytes_be_with_len (x_pub , group . p . len ()) ; Ok (StartedKeyExchange :: Single (Box :: new (ActiveFfdheKx { x_pub , x , p , group , named_group : self . 0 , }))) } fn ffdhe_group (& self) -> Option < FfdheGroup < 'static > > { Some (self . 1) } fn name (& self) -> NamedGroup { self . 0 } }
};
}
