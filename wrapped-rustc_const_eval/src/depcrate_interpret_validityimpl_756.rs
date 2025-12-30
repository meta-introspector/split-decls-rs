// Generated macro for impl_756 (impl)
macro_rules! Depcrate_interpret_validityimpl_756 {
() => {
// Module: crate::interpret::validity
// Provides: {"impl_756"}
// Dependencies: {}
impl RangeSet { fn add_range (& mut self , offset : Size , size : Size) { if size . bytes () == 0 { return ; } let v = & mut self . 0 ; let idx = v . partition_point (| & (other_offset , other_size) | other_offset + other_size < offset) ; if let Some (& (other_offset , other_size)) = v . get (idx) && offset + size >= other_offset { let new_start = other_offset . min (offset) ; let mut new_end = (other_offset + other_size) . max (offset + size) ; let mut scan_right = 1 ; while let Some (& (next_offset , next_size)) = v . get (idx + scan_right) && new_end >= next_offset { new_end = new_end . max (next_offset + next_size) ; scan_right += 1 ; } v [idx] = (new_start , new_end - new_start) ; if scan_right > 1 { drop (v . drain ((idx + 1) .. (idx + scan_right))) ; } } else { v . insert (idx , (offset , size)) ; } } }
};
}
