// Generated macro for impl_500 (impl)
macro_rules! Depcrateimpl_500 {
() => {
// Module: crate
// Provides: {"impl_500"}
// Dependencies: {}
impl FromIterator < CompactString > for String { fn from_iter < T : IntoIterator < Item = CompactString > > (iter : T) -> Self { let mut iterator = iter . into_iter () ; match iterator . next () { None => String :: new () , Some (buf) => { let mut buf = buf . into_string () ; buf . extend (iterator) ; buf } } } }
};
}
