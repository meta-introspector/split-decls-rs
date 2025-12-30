// Generated macro for TerminationInfo (enum)
macro_rules! Depcrate_diagnosticsTerminationInfo {
() => {
// Module: crate::diagnostics
// Provides: {"TerminationInfo"}
// Dependencies: {}
# [doc = " Details of premature program termination."] pub enum TerminationInfo { Exit { code : i32 , leak_check : bool , } , Abort (String) , # [doc = " Miri was interrupted by a Ctrl+C from the user"] Interrupted , UnsupportedInIsolation (String) , StackedBorrowsUb { msg : String , help : Vec < String > , history : Option < TagHistory > , } , TreeBorrowsUb { title : String , details : Vec < String > , history : tree_diagnostics :: HistoryData , } , Int2PtrWithStrictProvenance , Deadlock , # [doc = " In GenMC mode, an execution can get stuck in certain cases. This is not an error."] GenmcStuckExecution , MultipleSymbolDefinitions { link_name : Symbol , first : SpanData , first_crate : Symbol , second : SpanData , second_crate : Symbol , } , SymbolShimClashing { link_name : Symbol , span : SpanData , } , DataRace { involves_non_atomic : bool , ptr : interpret :: Pointer < AllocId > , op1 : RacingOp , op2 : RacingOp , extra : Option < & 'static str > , retag_explain : bool , } , UnsupportedForeignItem (String) , }
};
}
