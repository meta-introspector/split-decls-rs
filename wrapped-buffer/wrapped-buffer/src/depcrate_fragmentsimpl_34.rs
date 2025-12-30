// Generated macro for impl_34 (impl)
macro_rules! Depcrate_fragmentsimpl_34 {
() => {
// Module: crate::fragments
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'sval , const N : usize > From < & 'sval [u8 ; N] > for BinaryBuf < 'sval > { fn from (fragment : & 'sval [u8 ; N]) -> Self { BinaryBuf { buf : FragmentBuf :: new (fragment) , } } }
};
}
