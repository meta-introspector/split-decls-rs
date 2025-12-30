// Generated macro for ALL (static)
macro_rules! Depcrate_corpus_runALL {
() => {
// Module: crate::corpus::run
// Provides: {"ALL"}
// Dependencies: {}
pub (crate) static ALL : & [Task] = & [# [cfg (feature = "archive")] Task { short_name : "SWTR" , description : "stream worktree" , execute_exclusive : false , execute : & WorktreeStream , } , Task { short_name : "OPNR" , description : "open repository (isolated)" , execute_exclusive : false , execute : & OpenRepo , } , Task { short_name : "POCN" , description : "packed object count" , execute_exclusive : false , execute : & CountPackedObjects , } , Task { short_name : "VERI" , description : "verify object database" , execute_exclusive : true , execute : & VerifyOdb , } ,] ;
};
}
