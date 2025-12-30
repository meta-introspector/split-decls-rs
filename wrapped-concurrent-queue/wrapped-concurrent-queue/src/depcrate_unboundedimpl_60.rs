// Generated macro for impl_60 (impl)
macro_rules! Depcrate_unboundedimpl_60 {
() => {
// Module: crate::unbounded
// Provides: {"impl_60"}
// Dependencies: {}
impl < T > Drop for Unbounded < T > { fn drop (& mut self) { let Self { head , tail } = self ; let Position { index : head , block } = & mut * * head ; head . with_mut (| & mut mut head | { tail . index . with_mut (| & mut mut tail | { head &= ! ((1 << SHIFT) - 1) ; tail &= ! ((1 << SHIFT) - 1) ; unsafe { while head != tail { let offset = (head >> SHIFT) % LAP ; if offset < BLOCK_CAP { block . with_mut (| block | { let slot = (* * block) . slots . get_unchecked (offset) ; slot . value . with_mut (| slot | { let value = & mut * slot ; value . as_mut_ptr () . drop_in_place () ; }) ; }) ; } else { block . with_mut (| block | { let next_block = (* * block) . next . with_mut (| next | * next) ; drop (Box :: from_raw (* block)) ; * block = next_block ; }) ; } head = head . wrapping_add (1 << SHIFT) ; } block . with_mut (| block | { if ! block . is_null () { drop (Box :: from_raw (* block)) ; } }) ; } }) ; }) ; } }
};
}
