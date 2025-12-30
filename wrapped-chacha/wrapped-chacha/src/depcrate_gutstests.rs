// Generated macro for tests (module)
macro_rules! Depcrate_gutstests {
() => {
// Module: crate::guts
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [doc = " Basic check that streamXX_eq is block-count invariant"] # [test] fn test_stream_eq () { let key = hex ! ("fa44478c59ca70538e3549096ce8b523232c50d9e8e8d10c203ef6c8d07098a5") ; let nonce = hex ! ("8d3a0d6d7827c00701020304") ; let mut a = ChaCha :: new (& key , & nonce) ; let b = a . clone () ; let mut out = [0u8 ; BLOCK] ; assert ! (a == b) ; assert ! (a . stream32_eq (& b)) ; assert ! (a . stream64_eq (& b)) ; a . refill (0 , & mut out) ; assert ! (a != b) ; assert ! (a . stream32_eq (& b)) ; assert ! (a . stream64_eq (& b)) ; } }
};
}
