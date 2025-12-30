// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Vcpu < 'a > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("Vcpu") . field ("fd" , & self . fd) . field ("vm" , & self . vm) . finish () } }
};
}
