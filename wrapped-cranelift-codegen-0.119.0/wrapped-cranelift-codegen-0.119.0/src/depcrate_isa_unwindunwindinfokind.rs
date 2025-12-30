// Generated macro for UnwindInfoKind (enum)
macro_rules! Depcrate_isa_unwindUnwindInfoKind {
() => {
// Module: crate::isa::unwind
// Provides: {"UnwindInfoKind"}
// Dependencies: {}
# [doc = " Expected unwind info type."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub enum UnwindInfoKind { # [doc = " No unwind info."] None , # [doc = " SystemV CIE/FDE unwind info."] # [cfg (feature = "unwind")] SystemV , # [doc = " Windows X64 Unwind info"] # [cfg (feature = "unwind")] Windows , }
};
}
