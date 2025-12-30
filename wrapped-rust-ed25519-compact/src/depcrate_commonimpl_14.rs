// Generated macro for impl_14 (impl)
macro_rules! Depcrate_commonimpl_14 {
() => {
// Module: crate::common
// Provides: {"impl_14"}
// Dependencies: {}
impl Mem { # [inline] pub fn wipe < T : Default > (mut x : impl AsMut < [T] >) { let x = x . as_mut () ; for i in 0 .. x . len () { unsafe { ptr :: write_volatile (x . as_mut_ptr () . add (i) , T :: default ()) ; } } atomic :: compiler_fence (atomic :: Ordering :: SeqCst) ; atomic :: fence (atomic :: Ordering :: SeqCst) ; } }
};
}
