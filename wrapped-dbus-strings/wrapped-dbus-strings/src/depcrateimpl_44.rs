// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a > From < & 'a SignatureSingle > for & 'a SignatureMulti { fn from (s : & 'a SignatureSingle) -> & 'a SignatureMulti { SignatureMulti :: new_unchecked (& s . 0) } }
};
}
