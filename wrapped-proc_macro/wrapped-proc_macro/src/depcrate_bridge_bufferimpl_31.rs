// Generated macro for impl_31 (impl)
macro_rules! Depcrate_bridge_bufferimpl_31 {
() => {
// Module: crate::bridge::buffer
// Provides: {"impl_31"}
// Dependencies: {}
impl Buffer { # [inline] pub (super) fn new () -> Self { Self :: default () } # [inline] pub (super) fn clear (& mut self) { self . len = 0 ; } # [inline] pub (super) fn take (& mut self) -> Self { mem :: take (self) } # [inline] pub (super) fn extend_from_array < const N : usize > (& mut self , xs : & [u8 ; N]) { if xs . len () > (self . capacity - self . len) { let b = self . take () ; * self = (b . reserve) (b , xs . len ()) ; } unsafe { xs . as_ptr () . copy_to_nonoverlapping (self . data . add (self . len) , xs . len ()) ; self . len += xs . len () ; } } # [inline] pub (super) fn extend_from_slice (& mut self , xs : & [u8]) { if xs . len () > (self . capacity - self . len) { let b = self . take () ; * self = (b . reserve) (b , xs . len ()) ; } unsafe { xs . as_ptr () . copy_to_nonoverlapping (self . data . add (self . len) , xs . len ()) ; self . len += xs . len () ; } } # [inline] pub (super) fn push (& mut self , v : u8) { if self . len == self . capacity { let b = self . take () ; * self = (b . reserve) (b , 1) ; } unsafe { * self . data . add (self . len) = v ; self . len += 1 ; } } }
};
}
