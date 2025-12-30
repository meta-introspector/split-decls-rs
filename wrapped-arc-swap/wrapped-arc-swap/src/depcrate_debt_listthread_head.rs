// Generated macro for THREAD_HEAD (static)
macro_rules! Depcrate_debt_listTHREAD_HEAD {
() => {
// Module: crate::debt::list
// Provides: {"THREAD_HEAD"}
// Dependencies: {}
# [cfg (feature = "experimental-thread-local")] # [thread_local] # [doc = " A debt node assigned to this thread."] static THREAD_HEAD : OnceCell < LocalNode > = OnceCell :: new () ;
};
}
