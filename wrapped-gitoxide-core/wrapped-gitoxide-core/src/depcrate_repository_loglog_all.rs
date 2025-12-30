// Generated macro for log_all (function)
macro_rules! Depcrate_repository_loglog_all {
() => {
// Module: crate::repository::log
// Provides: {"log_all"}
// Dependencies: {}
fn log_all (repo : gix :: Repository , out : & mut dyn std :: io :: Write) -> Result < () , anyhow :: Error > { let head = repo . head () ? . peel_to_commit () ? ; let topo = gix :: traverse :: commit :: topo :: Builder :: from_iters (& repo . objects , [head . id] , None :: < Vec < gix :: ObjectId > >) . build () ? ; for info in topo { let info = info ? ; write_info (& repo , & mut * out , & info) ? ; } Ok (()) }
};
}
