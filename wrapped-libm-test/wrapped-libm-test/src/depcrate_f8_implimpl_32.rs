// Generated macro for impl_32 (impl)
macro_rules! Depcrate_f8_implimpl_32 {
() => {
// Module: crate::f8_impl
// Provides: {"impl_32"}
// Dependencies: {}
impl cmp :: PartialOrd for f8 { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { let inf_rep = f8 :: EXP_MASK ; let a_abs = self . abs () . to_bits () ; let b_abs = other . abs () . to_bits () ; if a_abs > inf_rep || b_abs > inf_rep { return None ; } if a_abs | b_abs == 0 { return Some (Ordering :: Equal) ; } let a_srep = self . to_bits_signed () ; let b_srep = other . to_bits_signed () ; let res = a_srep . cmp (& b_srep) ; if a_srep & b_srep >= 0 { Some (res) } else { Some (res . reverse ()) } } }
};
}
