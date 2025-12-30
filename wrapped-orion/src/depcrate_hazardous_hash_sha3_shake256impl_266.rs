// Generated macro for impl_266 (impl)
macro_rules! Depcrate_hazardous_hash_sha3_shake256impl_266 {
() => {
// Module: crate::hazardous::hash::sha3::shake256
// Provides: {"impl_266"}
// Dependencies: {}
impl Shake256 { # [doc = " Initialize a `Shake256` struct."] pub fn new () -> Self { Self { _state : Shake :: < SHAKE_256_RATE > :: _new (64) , } } # [doc = " Reset to `new()` state."] pub fn reset (& mut self) { self . _state . _reset () ; } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Update state with `data`. This can be called multiple times."] pub fn absorb (& mut self , data : & [u8]) -> Result < () , UnknownCryptoError > { self . _state . _absorb (data) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Squeeze output of the XOF into `dest`. This can be called multiple times."] pub fn squeeze (& mut self , dest : & mut [u8]) -> Result < () , UnknownCryptoError > { self . _state . _squeeze (dest) } }
};
}
