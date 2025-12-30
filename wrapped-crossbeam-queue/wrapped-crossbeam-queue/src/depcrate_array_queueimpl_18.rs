// Generated macro for impl_18 (impl)
macro_rules! Depcrate_array_queueimpl_18 {
() => {
// Module: crate::array_queue
// Provides: {"impl_18"}
// Dependencies: {}
impl < T > Drop for ArrayQueue < T > { fn drop (& mut self) { if mem :: needs_drop :: < T > () { let head = * self . head . get_mut () ; let tail = * self . tail . get_mut () ; let hix = head & (self . one_lap - 1) ; let tix = tail & (self . one_lap - 1) ; let len = if hix < tix { tix - hix } else if hix > tix { self . capacity () - hix + tix } else if tail == head { 0 } else { self . capacity () } ; for i in 0 .. len { let index = if hix + i < self . capacity () { hix + i } else { hix + i - self . capacity () } ; unsafe { debug_assert ! (index < self . buffer . len ()) ; let slot = self . buffer . get_unchecked_mut (index) ; (* slot . value . get ()) . assume_init_drop () ; } } } } }
};
}
