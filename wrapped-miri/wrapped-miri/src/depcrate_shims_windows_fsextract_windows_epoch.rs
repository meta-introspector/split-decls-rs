// Generated macro for extract_windows_epoch (function)
macro_rules! Depcrate_shims_windows_fsextract_windows_epoch {
() => {
// Module: crate::shims::windows::fs
// Provides: {"extract_windows_epoch"}
// Dependencies: {}
# [doc = " Windows FILETIME is measured in 100-nanosecs since 1601"] fn extract_windows_epoch < 'tcx > (ecx : & MiriInterpCx < 'tcx > , time : io :: Result < SystemTime > ,) -> InterpResult < 'tcx , Option < (u32 , u32) > > { match time . ok () { Some (time) => { let duration = ecx . system_time_since_windows_epoch (& time) ? ; let duration_ticks = ecx . windows_ticks_for (duration) ? ; # [expect (clippy :: as_conversions)] interp_ok (Some ((duration_ticks as u32 , (duration_ticks >> 32) as u32))) } None => interp_ok (None) , } }
};
}
