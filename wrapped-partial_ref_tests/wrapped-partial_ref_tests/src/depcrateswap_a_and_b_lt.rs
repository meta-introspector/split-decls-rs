// Generated macro for swap_a_and_b_lt (function)
macro_rules! Depcrateswap_a_and_b_lt {
() => {
// Module: crate
// Provides: {"swap_a_and_b_lt"}
// Dependencies: {}
pub fn swap_a_and_b_lt < 'a , T > (mut partial_ref : partial ! ('a T , mut PartA , mut PartB)) where T : PartialRefTarget , T : HasPart < PartA > , T : HasPart < PartB > , { split_borrow ! (a , b = & (mut PartA) partial_ref) ; std :: mem :: swap (a . part_mut (PartA) , b . part_mut (PartB)) }
};
}
