// Generated macro for BarrettReduce (trait)
macro_rules! Depcrate_algebraBarrettReduce {
() => {
// Module: crate::algebra
// Provides: {"BarrettReduce"}
// Dependencies: {}
pub (crate) trait BarrettReduce : Unsigned { const SHIFT : usize ; const MULTIPLIER : u64 ; fn reduce (x : u32) -> u32 { let m = Self :: U64 ; let x : u64 = x . into () ; let quotient = (x * Self :: MULTIPLIER) >> Self :: SHIFT ; let remainder = x - quotient * m ; if remainder < m { Truncate :: truncate (remainder) } else { Truncate :: truncate (remainder - m) } } }
};
}
