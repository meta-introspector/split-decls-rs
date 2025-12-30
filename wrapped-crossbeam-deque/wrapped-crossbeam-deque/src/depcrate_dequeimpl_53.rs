// Generated macro for impl_53 (impl)
macro_rules! Depcrate_dequeimpl_53 {
() => {
// Module: crate::deque
// Provides: {"impl_53"}
// Dependencies: {}
impl < T > Drop for Injector < T > { fn drop (& mut self) { let mut head = * self . head . index . get_mut () ; let mut tail = * self . tail . index . get_mut () ; let mut block = * self . head . block . get_mut () ; head &= ! ((1 << SHIFT) - 1) ; tail &= ! ((1 << SHIFT) - 1) ; unsafe { while head != tail { let offset = (head >> SHIFT) % LAP ; if offset < BLOCK_CAP { let slot = (* block) . slots . get_unchecked (offset) ; (* slot . task . get ()) . assume_init_drop () ; } else { let next = * (* block) . next . get_mut () ; drop (Box :: from_raw (block)) ; block = next ; } head = head . wrapping_add (1 << SHIFT) ; } drop (Box :: from_raw (block)) ; } } }
};
}
