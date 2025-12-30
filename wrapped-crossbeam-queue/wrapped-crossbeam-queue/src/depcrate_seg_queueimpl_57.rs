// Generated macro for impl_57 (impl)
macro_rules! Depcrate_seg_queueimpl_57 {
() => {
// Module: crate::seg_queue
// Provides: {"impl_57"}
// Dependencies: {}
impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { let value = & mut self . value ; let head = * value . head . index . get_mut () ; let tail = * value . tail . index . get_mut () ; if head >> SHIFT == tail >> SHIFT { None } else { let block = * value . head . block . get_mut () ; let offset = (head >> SHIFT) % LAP ; let item = unsafe { let slot = (* block) . slots . get_unchecked (offset) ; slot . value . get () . read () . assume_init () } ; if offset + 1 == BLOCK_CAP { unsafe { let next = * (* block) . next . get_mut () ; drop (Box :: from_raw (block)) ; * value . head . block . get_mut () = next ; } * value . head . index . get_mut () = head . wrapping_add (2 << SHIFT) ; debug_assert_eq ! ((* value . head . index . get_mut () >> SHIFT) % LAP , 0) ; } else { * value . head . index . get_mut () = head . wrapping_add (1 << SHIFT) ; } Some (item) } } }
};
}
