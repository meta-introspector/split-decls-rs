// Generated macro for ArgsAccumulator (struct)
macro_rules! Depcrate_machinst_abiArgsAccumulator {
() => {
// Module: crate::machinst::abi
// Provides: {"ArgsAccumulator"}
// Dependencies: {}
# [doc = " Used as an out-parameter to accumulate a sequence of `ABIArg`s in"] # [doc = " `ABIMachineSpec::compute_arg_locs`. Wraps the shared allocation for all"] # [doc = " `ABIArg`s in `SigSet` and exposes just the args for the current"] # [doc = " `compute_arg_locs` call."] pub struct ArgsAccumulator < 'a > { sig_set_abi_args : & 'a mut Vec < ABIArg > , start : usize , non_formal_flag : bool , }
};
}
