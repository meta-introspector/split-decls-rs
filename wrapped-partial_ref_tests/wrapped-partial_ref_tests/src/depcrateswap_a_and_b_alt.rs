// Generated macro for swap_a_and_b_alt (function)
macro_rules! Depcrateswap_a_and_b_alt {
() => {
// Module: crate
// Provides: {"swap_a_and_b_alt"}
// Dependencies: {}
pub fn swap_a_and_b_alt < T > (mut partial_ref : partial ! (T , mut PartA , mut PartB)) where T : PartialRefTarget , T : HasPart < PartA > , T : HasPart < PartB > , { let (a , mut partial_ref) = partial_ref . split_part_mut (PartA) ; let (b , _) = partial_ref . split_part_mut (PartB) ; std :: mem :: swap (a , b) ; }
};
}
