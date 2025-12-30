// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a , T > From < & 'a COption < T > > for COption < & 'a T > { fn from (o : & 'a COption < T >) -> COption < & 'a T > { o . as_ref () } }
};
}
