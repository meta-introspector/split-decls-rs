// Generated macro for impl_482 (impl)
macro_rules! Depcrate_ioimpl_482 {
() => {
// Module: crate::io
// Provides: {"impl_482"}
// Dependencies: {}
impl < I , T : AsyncWrite + Unpin , U : Encoder < I > > Drop for FramedWrite < I , T , U > { fn drop (& mut self) { let mut async_writer = self . inner . 1 . borrow_mut () ; let inner = self . inner . 0 . borrow_mut () ; if ! inner . buffer . is_empty () { drop (async_writer . write (& inner . buffer)) ; drop (async_writer . flush ()) ; } } }
};
}
