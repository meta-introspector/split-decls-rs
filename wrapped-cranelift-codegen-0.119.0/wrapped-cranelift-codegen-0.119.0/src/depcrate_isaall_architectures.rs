// Generated macro for ALL_ARCHITECTURES (const)
macro_rules! Depcrate_isaALL_ARCHITECTURES {
() => {
// Module: crate::isa
// Provides: {"ALL_ARCHITECTURES"}
// Dependencies: {}
# [doc = " The string names of all the supported, but possibly not enabled, architectures. The elements of"] # [doc = " this slice are suitable to be passed to the [lookup_by_name] function to obtain the default"] # [doc = " configuration for that architecture."] pub const ALL_ARCHITECTURES : & [& str] = & ["x86_64" , "aarch64" , "s390x" , "riscv64"] ;
};
}
