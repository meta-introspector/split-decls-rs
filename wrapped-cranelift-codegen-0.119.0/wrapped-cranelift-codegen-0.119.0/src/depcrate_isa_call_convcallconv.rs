// Generated macro for CallConv (enum)
macro_rules! Depcrate_isa_call_convCallConv {
() => {
// Module: crate::isa::call_conv
// Provides: {"CallConv"}
// Dependencies: {}
# [doc = " Calling convention identifiers."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum CallConv { # [doc = " Best performance, not ABI-stable."] Fast , # [doc = " Smallest caller code size, not ABI-stable."] Cold , # [doc = " Supports tail calls, not ABI-stable."] Tail , # [doc = " System V-style convention used on many platforms."] SystemV , # [doc = " Windows \"fastcall\" convention, also used for x64 and ARM."] WindowsFastcall , # [doc = " Mac aarch64 calling convention, which is a tweaked aarch64 ABI."] AppleAarch64 , # [doc = " Specialized convention for the probestack function."] Probestack , # [doc = " The winch calling convention, not ABI-stable."] # [doc = ""] # [doc = " The main difference to SystemV is that the winch calling convention"] # [doc = " defines no callee-save registers, and restricts the number of return"] # [doc = " registers to one integer, and one floating point."] Winch , }
};
}
