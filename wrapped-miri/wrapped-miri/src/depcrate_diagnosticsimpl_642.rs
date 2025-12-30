// Generated macro for impl_642 (impl)
macro_rules! Depcrate_diagnosticsimpl_642 {
() => {
// Module: crate::diagnostics
// Provides: {"impl_642"}
// Dependencies: {}
impl fmt :: Display for TerminationInfo { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use TerminationInfo :: * ; match self { Exit { code , .. } => write ! (f , "the evaluated program completed with exit code {code}") , Abort (msg) => write ! (f , "{msg}") , Interrupted => write ! (f , "interpretation was interrupted") , UnsupportedInIsolation (msg) => write ! (f , "{msg}") , Int2PtrWithStrictProvenance => write ! (f , "integer-to-pointer casts and `ptr::with_exposed_provenance` are not supported with `-Zmiri-strict-provenance`") , StackedBorrowsUb { msg , .. } => write ! (f , "{msg}") , TreeBorrowsUb { title , .. } => write ! (f , "{title}") , Deadlock => write ! (f , "the evaluated program deadlocked") , GenmcStuckExecution => write ! (f , "GenMC determined that the execution got stuck") , MultipleSymbolDefinitions { link_name , .. } => write ! (f , "multiple definitions of symbol `{link_name}`") , SymbolShimClashing { link_name , .. } => write ! (f , "found `{link_name}` symbol definition that clashes with a built-in shim" ,) , DataRace { involves_non_atomic , ptr , op1 , op2 , .. } => write ! (f , "{} detected between (1) {} on {} and (2) {} on {} at {ptr:?}" , if * involves_non_atomic { "Data race" } else { "Race condition" } , op1 . action , op1 . thread_info , op2 . action , op2 . thread_info) , UnsupportedForeignItem (msg) => write ! (f , "{msg}") , } } }
};
}
