// Generated macro for u32 (function)
macro_rules! Depcrateu32 {
() => {
// Module: crate
// Provides: {"u32"}
// Dependencies: {}
# [doc = " Get random `u32` from the system's preferred random number source."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn main() -> Result<(), getrandom::Error> {"] # [doc = " let rng_seed = getrandom::u32()?;"] # [doc = " # Ok(()) }"] # [doc = " ```"] # [inline] pub fn u32 () -> Result < u32 , Error > { backends :: inner_u32 () }
};
}
