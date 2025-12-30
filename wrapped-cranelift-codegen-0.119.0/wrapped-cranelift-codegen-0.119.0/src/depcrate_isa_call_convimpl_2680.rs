// Generated macro for impl_2680 (impl)
macro_rules! Depcrate_isa_call_convimpl_2680 {
() => {
// Module: crate::isa::call_conv
// Provides: {"impl_2680"}
// Dependencies: {}
impl fmt :: Display for CallConv { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match * self { Self :: Fast => "fast" , Self :: Cold => "cold" , Self :: Tail => "tail" , Self :: SystemV => "system_v" , Self :: WindowsFastcall => "windows_fastcall" , Self :: AppleAarch64 => "apple_aarch64" , Self :: Probestack => "probestack" , Self :: Winch => "winch" , }) } }
};
}
