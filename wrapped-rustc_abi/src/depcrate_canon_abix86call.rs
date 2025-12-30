// Generated macro for X86Call (enum)
macro_rules! Depcrate_canon_abiX86Call {
() => {
// Module: crate::canon_abi
// Provides: {"X86Call"}
// Dependencies: {}
# [doc = " ABIs defined for x86-{32,64}"] # [doc = ""] # [doc = " One of SysV64 or Win64 may alias the C ABI, and arguably Win64 is cross-platform now?"] # [derive (Clone , Copy , Debug)] # [derive (PartialOrd , Ord , PartialEq , Eq , Hash)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub enum X86Call { # [doc = " \"fastcall\" has both GNU and Windows variants"] Fastcall , # [doc = " \"stdcall\" has both GNU and Windows variants"] Stdcall , SysV64 , Thiscall , Vectorcall , Win64 , }
};
}
