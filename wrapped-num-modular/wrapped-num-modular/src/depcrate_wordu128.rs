// Generated macro for u128 (module)
macro_rules! Depcrate_wordu128 {
() => {
// Module: crate::word
// Provides: {"u128"}
// Dependencies: {}
pub mod u128 { use crate :: double :: udouble ; pub type Word = u128 ; pub type DoubleWord = udouble ; # [inline] pub const fn extend (word : Word) -> DoubleWord { udouble { lo : word , hi : 0 } } # [inline (always)] pub const fn low (dw : DoubleWord) -> Word { dw . lo } # [inline (always)] pub const fn high (dw : DoubleWord) -> Word { dw . hi } # [inline] pub const fn split (dw : DoubleWord) -> (Word , Word) { (dw . lo , dw . hi) } # [inline] pub const fn merge (low : Word , high : Word) -> DoubleWord { udouble { lo : low , hi : high } } # [inline] pub const fn wmul (a : Word , b : Word) -> DoubleWord { udouble :: widening_mul (a , b) } # [inline] pub const fn wsqr (a : Word) -> DoubleWord { udouble :: widening_square (a) } # [inline] pub fn nrem (n : DoubleWord , d : Word) -> Word { n % d } }
};
}
