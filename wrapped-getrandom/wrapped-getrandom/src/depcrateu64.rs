// Generated macro for u64 (function)
macro_rules! Depcrateu64 {
() => {
// Module: crate
// Provides: {"u64"}
// Dependencies: {}
# [doc = " Get random `u64` from the system's preferred random number source."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn main() -> Result<(), getrandom::Error> {"] # [doc = " let rng_seed = getrandom::u64()?;"] # [doc = " # Ok(()) }"] # [doc = " ```"] # [inline] pub fn u64 () -> Result < u64 , Error > { backends :: inner_u64 () }
};
}
