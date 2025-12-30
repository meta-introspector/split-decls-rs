// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < T > From < Option < T > > for COption < T > { fn from (option : Option < T >) -> Self { match option { Some (value) => COption :: Some (value) , None => COption :: None , } } }
};
}
