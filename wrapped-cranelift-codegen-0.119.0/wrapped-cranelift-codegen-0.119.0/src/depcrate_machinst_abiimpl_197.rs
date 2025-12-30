// Generated macro for impl_197 (impl)
macro_rules! Depcrate_machinst_abiimpl_197 {
() => {
// Module: crate::machinst::abi
// Provides: {"impl_197"}
// Dependencies: {}
impl < 'a > ArgsAccumulator < 'a > { fn new (sig_set_abi_args : & 'a mut Vec < ABIArg >) -> Self { let start = sig_set_abi_args . len () ; ArgsAccumulator { sig_set_abi_args , start , non_formal_flag : false , } } # [inline] pub fn push (& mut self , arg : ABIArg) { debug_assert ! (! self . non_formal_flag) ; self . sig_set_abi_args . push (arg) } # [inline] pub fn push_non_formal (& mut self , arg : ABIArg) { self . non_formal_flag = true ; self . sig_set_abi_args . push (arg) } # [inline] pub fn args (& self) -> & [ABIArg] { & self . sig_set_abi_args [self . start ..] } # [inline] pub fn args_mut (& mut self) -> & mut [ABIArg] { & mut self . sig_set_abi_args [self . start ..] } }
};
}
