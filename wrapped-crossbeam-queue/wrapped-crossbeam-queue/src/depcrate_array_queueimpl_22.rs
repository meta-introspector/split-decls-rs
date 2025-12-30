// Generated macro for impl_22 (impl)
macro_rules! Depcrate_array_queueimpl_22 {
() => {
// Module: crate::array_queue
// Provides: {"impl_22"}
// Dependencies: {}
impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { let value = & mut self . value ; let head = * value . head . get_mut () ; if value . head . get_mut () != value . tail . get_mut () { let index = head & (value . one_lap - 1) ; let lap = head & ! (value . one_lap - 1) ; let val = unsafe { debug_assert ! (index < value . buffer . len ()) ; let slot = value . buffer . get_unchecked_mut (index) ; slot . value . get () . read () . assume_init () } ; let new = if index + 1 < value . capacity () { head + 1 } else { lap . wrapping_add (value . one_lap) } ; * value . head . get_mut () = new ; Some (val) } else { None } } }
};
}
