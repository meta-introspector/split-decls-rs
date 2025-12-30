// Generated macro for macro_123 (macro)
macro_rules! Depcrate_debt_listmacro_123 {
() => {
// Module: crate::debt::list
// Provides: {"macro_123"}
// Dependencies: {}
# [cfg (not (feature = "experimental-thread-local"))] thread_local ! { # [doc = " A debt node assigned to this thread."] static THREAD_HEAD : LocalNode = LocalNode { node : Cell :: new (None) , fast : FastLocal :: default () , helping : HelpingLocal :: default () , } ; }
};
}
