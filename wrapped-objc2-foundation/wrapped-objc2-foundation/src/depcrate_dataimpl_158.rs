// Generated macro for impl_158 (impl)
macro_rules! Depcrate_dataimpl_158 {
() => {
// Module: crate::data
// Provides: {"impl_158"}
// Dependencies: {}
impl < 'a > Iter < 'a > { fn new (data : & 'a NSData) -> Self { Self { p : PhantomData , # [cfg (debug_assertions)] data , # [cfg (debug_assertions)] length : data . length () , bytes : data . to_vec () . into_iter () , } } }
};
}
