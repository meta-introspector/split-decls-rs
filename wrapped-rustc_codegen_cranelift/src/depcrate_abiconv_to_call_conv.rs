// Generated macro for conv_to_call_conv (function)
macro_rules! Depcrate_abiconv_to_call_conv {
() => {
// Module: crate::abi
// Provides: {"conv_to_call_conv"}
// Dependencies: {}
pub (crate) fn conv_to_call_conv (sess : & Session , c : CanonAbi , default_call_conv : CallConv ,) -> CallConv { match c { CanonAbi :: Rust | CanonAbi :: C => default_call_conv , CanonAbi :: RustCold => CallConv :: Cold , CanonAbi :: Custom => default_call_conv , CanonAbi :: X86 (x86_call) => match x86_call { X86Call :: SysV64 => CallConv :: SystemV , X86Call :: Win64 => CallConv :: WindowsFastcall , _ => default_call_conv , } , CanonAbi :: Interrupt (_) | CanonAbi :: Arm (_) => { sess . dcx () . fatal ("call conv {c:?} is not yet implemented") } CanonAbi :: GpuKernel => { unreachable ! ("tried to use {c:?} call conv which only exists on an unsupported target") } } }
};
}
