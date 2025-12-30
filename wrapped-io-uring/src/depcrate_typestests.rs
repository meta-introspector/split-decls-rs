// Generated macro for tests (module)
macro_rules! Depcrate_typestests {
() => {
// Module: crate::types
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: time :: Duration ; use crate :: types :: sealed :: Target ; use super :: * ; # [test] fn timespec_from_duration_converts_correctly () { let duration = Duration :: new (2 , 500) ; let timespec = Timespec :: from (duration) ; assert_eq ! (timespec . 0 . tv_sec as u64 , duration . as_secs ()) ; assert_eq ! (timespec . 0 . tv_nsec as u32 , duration . subsec_nanos ()) ; } # [test] fn test_cancel_builder_flags () { let cb = CancelBuilder :: any () ; assert_eq ! (cb . flags , AsyncCancelFlags :: ANY) ; let mut cb = CancelBuilder :: user_data (42) ; assert_eq ! (cb . flags , AsyncCancelFlags :: empty ()) ; assert_eq ! (cb . user_data , Some (42)) ; assert ! (cb . fd . is_none ()) ; cb = cb . all () ; assert_eq ! (cb . flags , AsyncCancelFlags :: ALL) ; let mut cb = CancelBuilder :: fd (Fd (42)) ; assert_eq ! (cb . flags , AsyncCancelFlags :: FD) ; assert ! (matches ! (cb . fd , Some (Target :: Fd (42)))) ; assert ! (cb . user_data . is_none ()) ; cb = cb . all () ; assert_eq ! (cb . flags , AsyncCancelFlags :: FD | AsyncCancelFlags :: ALL) ; let mut cb = CancelBuilder :: fd (Fixed (42)) ; assert_eq ! (cb . flags , AsyncCancelFlags :: FD | AsyncCancelFlags :: FD_FIXED) ; assert ! (matches ! (cb . fd , Some (Target :: Fixed (42)))) ; assert ! (cb . user_data . is_none ()) ; cb = cb . all () ; assert_eq ! (cb . flags , AsyncCancelFlags :: FD | AsyncCancelFlags :: FD_FIXED | AsyncCancelFlags :: ALL) ; } }
};
}
