// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a > From < & 'a AccountView > for InstructionAccount < 'a > { fn from (account : & 'a AccountView) -> Self { InstructionAccount :: new (account . address () , account . is_writable () , account . is_signer () ,) } }
};
}
