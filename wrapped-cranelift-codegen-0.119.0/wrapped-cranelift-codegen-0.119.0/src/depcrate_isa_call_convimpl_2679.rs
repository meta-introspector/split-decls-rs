// Generated macro for impl_2679 (impl)
macro_rules! Depcrate_isa_call_convimpl_2679 {
() => {
// Module: crate::isa::call_conv
// Provides: {"impl_2679"}
// Dependencies: {}
impl CallConv { # [doc = " Return the default calling convention for the given target triple."] pub fn triple_default (triple : & Triple) -> Self { match triple . default_calling_convention () { Ok (CallingConvention :: SystemV) | Err (()) => Self :: SystemV , Ok (CallingConvention :: AppleAarch64) => Self :: AppleAarch64 , Ok (CallingConvention :: WindowsFastcall) => Self :: WindowsFastcall , Ok (unimp) => unimplemented ! ("calling convention: {:?}" , unimp) , } } # [doc = " Returns the calling convention used for libcalls according to the current flags."] pub fn for_libcall (flags : & settings :: Flags , default_call_conv : CallConv) -> Self { match flags . libcall_call_conv () { LibcallCallConv :: IsaDefault => default_call_conv , LibcallCallConv :: Fast => Self :: Fast , LibcallCallConv :: Cold => Self :: Cold , LibcallCallConv :: SystemV => Self :: SystemV , LibcallCallConv :: WindowsFastcall => Self :: WindowsFastcall , LibcallCallConv :: AppleAarch64 => Self :: AppleAarch64 , LibcallCallConv :: Probestack => Self :: Probestack , } } # [doc = " Does this calling convention support tail calls?"] pub fn supports_tail_calls (& self) -> bool { match self { CallConv :: Tail => true , _ => false , } } }
};
}
