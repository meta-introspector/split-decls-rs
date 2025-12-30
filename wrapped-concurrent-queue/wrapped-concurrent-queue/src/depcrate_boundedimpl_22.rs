// Generated macro for impl_22 (impl)
macro_rules! Depcrate_boundedimpl_22 {
() => {
// Module: crate::bounded
// Provides: {"impl_22"}
// Dependencies: {}
impl < T > Drop for Bounded < T > { fn drop (& mut self) { let Self { head , tail , buffer , mark_bit , .. } = self ; let mark_bit = * mark_bit ; head . with_mut (| & mut head | { tail . with_mut (| & mut tail | { let hix = head & (mark_bit - 1) ; let tix = tail & (mark_bit - 1) ; let len = if hix < tix { tix - hix } else if hix > tix { buffer . len () - hix + tix } else if (tail & ! mark_bit) == head { 0 } else { buffer . len () } ; for i in 0 .. len { let index = if hix + i < buffer . len () { hix + i } else { hix + i - buffer . len () } ; let slot = & buffer [index] ; slot . value . with_mut (| slot | unsafe { let value = & mut * slot ; value . as_mut_ptr () . drop_in_place () ; }) ; } }) ; }) ; } }
};
}
