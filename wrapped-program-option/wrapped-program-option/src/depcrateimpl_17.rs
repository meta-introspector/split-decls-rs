// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a , T > From < & 'a mut COption < T > > for COption < & 'a mut T > { fn from (o : & 'a mut COption < T >) -> COption < & 'a mut T > { o . as_mut () } }
};
}
