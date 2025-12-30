// Generated macro for impl_669 (impl)
macro_rules! Depcrate_naive_timeimpl_669 {
() => {
// Module: crate::naive::time
// Provides: {"impl_669"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl arbitrary :: Arbitrary < '_ > for NaiveTime { fn arbitrary (u : & mut arbitrary :: Unstructured) -> arbitrary :: Result < NaiveTime > { let mins = u . int_in_range (0 ..= 1439) ? ; let mut secs = u . int_in_range (0 ..= 60) ? ; let mut nano = u . int_in_range (0 ..= 999_999_999) ? ; if secs == 60 { secs = 59 ; nano += 1_000_000_000 ; } let time = NaiveTime :: from_num_seconds_from_midnight_opt (mins * 60 + secs , nano) . expect ("Could not generate a valid chrono::NaiveTime. It looks like implementation of Arbitrary for NaiveTime is erroneous.") ; Ok (time) } }
};
}
