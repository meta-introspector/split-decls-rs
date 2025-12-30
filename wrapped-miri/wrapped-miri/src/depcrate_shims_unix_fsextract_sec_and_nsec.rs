// Generated macro for extract_sec_and_nsec (function)
macro_rules! Depcrate_shims_unix_fsextract_sec_and_nsec {
() => {
// Module: crate::shims::unix::fs
// Provides: {"extract_sec_and_nsec"}
// Dependencies: {}
# [doc = " Extracts the number of seconds and nanoseconds elapsed between `time` and the unix epoch when"] # [doc = " `time` is Ok. Returns `None` if `time` is an error. Fails if `time` happens before the unix"] # [doc = " epoch."] fn extract_sec_and_nsec < 'tcx > (time : std :: io :: Result < SystemTime > ,) -> InterpResult < 'tcx , Option < (u64 , u32) > > { match time . ok () { Some (time) => { let duration = system_time_to_duration (& time) ? ; interp_ok (Some ((duration . as_secs () , duration . subsec_nanos ()))) } None => interp_ok (None) , } }
};
}
