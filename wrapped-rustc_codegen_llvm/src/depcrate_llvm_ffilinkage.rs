// Generated macro for Linkage (enum)
macro_rules! Depcrate_llvm_ffiLinkage {
() => {
// Module: crate::llvm::ffi
// Provides: {"Linkage"}
// Dependencies: {}
# [doc = " Must match the layout of `LLVMLinkage`."] # [derive (Copy , Clone , PartialEq , TryFromU32)] # [repr (C)] pub (crate) enum Linkage { ExternalLinkage = 0 , AvailableExternallyLinkage = 1 , LinkOnceAnyLinkage = 2 , LinkOnceODRLinkage = 3 , # [deprecated = "marked obsolete by LLVM"] LinkOnceODRAutoHideLinkage = 4 , WeakAnyLinkage = 5 , WeakODRLinkage = 6 , AppendingLinkage = 7 , InternalLinkage = 8 , PrivateLinkage = 9 , # [deprecated = "marked obsolete by LLVM"] DLLImportLinkage = 10 , # [deprecated = "marked obsolete by LLVM"] DLLExportLinkage = 11 , ExternalWeakLinkage = 12 , # [deprecated = "marked obsolete by LLVM"] GhostLinkage = 13 , CommonLinkage = 14 , LinkerPrivateLinkage = 15 , LinkerPrivateWeakLinkage = 16 , }
};
}
