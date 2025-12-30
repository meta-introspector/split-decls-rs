// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < OutSize > CustomizedInit for Blake2b < OutSize > where OutSize : ArraySize + IsLessOrEqual < U64 , Output = True > , { fn new_customized (customization : & [u8]) -> Self { Self { core : CustomizedInit :: new_customized (customization) , buffer : Default :: default () , } } }
};
}
