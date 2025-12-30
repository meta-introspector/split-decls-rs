// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl < OutSize > CustomizedInit for Blake2s < OutSize > where OutSize : ArraySize + IsLessOrEqual < U32 , Output = True > , { fn new_customized (customization : & [u8]) -> Self { Self { core : CustomizedInit :: new_customized (customization) , buffer : Default :: default () , } } }
};
}
