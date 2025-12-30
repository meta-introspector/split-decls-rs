// Generated macro for impl_145 (impl)
macro_rules! Depcrate_int_bigimpl_145 {
() => {
// Module: crate::int::big
// Provides: {"impl_145"}
// Dependencies: {}
impl ops :: Shr < u32 > for u256 { type Output = Self ; fn shr (self , rhs : u32) -> Self :: Output { assert ! (rhs < Self :: BITS , "attempted to shift right with overflow") ; if rhs == 0 { return self ; } let mut ret = self ; let byte_shift = rhs / 64 ; let bit_shift = rhs % 64 ; for idx in 0 .. 4 { let base_idx = idx + byte_shift as usize ; let Some (base) = ret . 0 . get (base_idx) else { ret . 0 [idx] = 0 ; continue ; } ; let mut new_val = base >> bit_shift ; if let Some (new) = ret . 0 . get (base_idx + 1) { new_val |= new . overflowing_shl (64 - bit_shift) . 0 ; } ret . 0 [idx] = new_val ; } ret } }
};
}
