// Generated macro for Prepare (struct)
macro_rules! Depcrate_remote_connection_fetchPrepare {
() => {
// Module: crate::remote::connection::fetch
// Provides: {"Prepare"}
// Dependencies: {}
# [doc = " A structure to hold the result of the handshake with the remote and configure the upcoming fetch operation."] pub struct Prepare < 'remote , 'repo , T > where T : Transport , { con : Option < Connection < 'remote , 'repo , T > > , ref_map : RefMap , dry_run : DryRun , reflog_message : Option < RefLogMessage > , write_packed_refs : WritePackedRefs , shallow : remote :: fetch :: Shallow , }
};
}
