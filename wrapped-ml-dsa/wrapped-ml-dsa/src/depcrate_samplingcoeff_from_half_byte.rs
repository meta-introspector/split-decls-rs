// Generated macro for coeff_from_half_byte (function)
macro_rules! Depcrate_samplingcoeff_from_half_byte {
() => {
// Module: crate::sampling
// Provides: {"coeff_from_half_byte"}
// Dependencies: {}
fn coeff_from_half_byte (b : u8 , eta : Eta) -> Option < Elem > { match eta { Eta :: Two if b < 15 => { let b = Int :: from (match b { b if b < 5 => b , b if b < 10 => b - 5 , _ => b - 10 , }) ; if b <= 2 { Some (Elem :: new (2 - b)) } else { Some (- Elem :: new (b - 2)) } } Eta :: Four if b < 9 => { let b = Int :: from (b) ; if b <= 4 { Some (Elem :: new (4 - b)) } else { Some (- Elem :: new (b - 4)) } } _ => None , } }
};
}
