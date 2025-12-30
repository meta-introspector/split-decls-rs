// Generated macro for TargetInfo (struct)
macro_rules! Depcrate_targetTargetInfo {
() => {
// Module: crate::target
// Provides: {"TargetInfo"}
// Dependencies: {}
# [doc = " Information specific to a `rustc` target."] # [doc = ""] # [doc = " See <https://doc.rust-lang.org/cargo/appendix/glossary.html#target>."] # [derive (Debug , PartialEq , Clone)] pub (crate) struct TargetInfo < 'a > { # [doc = " The full architecture, including the subarchitecture."] # [doc = ""] # [doc = " This differs from `cfg!(target_arch)`, which only specifies the"] # [doc = " overall architecture, which is too coarse for certain cases."] pub full_arch : & 'a str , # [doc = " The overall target architecture."] # [doc = ""] # [doc = " This is the same as the value of `cfg!(target_arch)`."] pub arch : & 'a str , # [doc = " The target vendor."] # [doc = ""] # [doc = " This is the same as the value of `cfg!(target_vendor)`."] pub vendor : & 'a str , # [doc = " The operating system, or `none` on bare-metal targets."] # [doc = ""] # [doc = " This is the same as the value of `cfg!(target_os)`."] pub os : & 'a str , # [doc = " The environment on top of the operating system."] # [doc = ""] # [doc = " This is the same as the value of `cfg!(target_env)`."] pub env : & 'a str , # [doc = " The ABI on top of the operating system."] # [doc = ""] # [doc = " This is the same as the value of `cfg!(target_abi)`."] pub abi : & 'a str , }
};
}
