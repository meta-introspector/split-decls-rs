// Generated macro for __getrandom_v03_custom (function)
macro_rules! Depcrate__getrandom_v03_custom {
() => {
// Module: crate
// Provides: {"__getrandom_v03_custom"}
// Dependencies: {}
# [cfg (getrandom_backend = "custom")] # [unsafe (no_mangle)] unsafe extern "Rust" fn __getrandom_v03_custom (dest : * mut u8 , len : usize ,) -> Result < () , getrandom :: Error > { for i in 0 .. len { unsafe { core :: ptr :: write (dest . add (i) , i as u8) } ; } Ok (()) }
};
}
