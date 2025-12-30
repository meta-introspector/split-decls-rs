// Generated macro for impl_156 (impl)
macro_rules! Depcrateimpl_156 {
() => {
// Module: crate
// Provides: {"impl_156"}
// Dependencies: {}
impl < 'a > Iterator for Zeroes < 'a > { type Item = usize ; # [inline] fn next (& mut self) -> Option < Self :: Item > { while self . bitset == 0 { self . bitset = ! * self . remaining_blocks . next () ? ; self . block_idx += BITS ; } let t = self . bitset & (0_usize) . wrapping_sub (self . bitset) ; let r = self . bitset . trailing_zeros () as usize ; self . bitset ^= t ; let bit = self . block_idx + r ; if bit < self . len { Some (bit) } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . len)) } }
};
}
