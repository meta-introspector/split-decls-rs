// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < 'a , T : iter :: ExactSizeIterator + Iterator < Item = & 'a u8 > > iter :: ExactSizeIterator for ToHexIter < T > { fn len (& self) -> usize { let mut len = self . inner . len () * 2 ; if self . live . is_some () { len += 1 ; } len } }
};
}
