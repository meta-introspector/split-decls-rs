// Generated macro for test_pattern (function)
macro_rules! Depcratetest_pattern {
() => {
// Module: crate
// Provides: {"test_pattern"}
// Dependencies: {}
# [doc = " Tests a specific pattern, returning measurement and scoring outcomes."] fn test_pattern (pattern : & Pattern , time_samples : & mut [(f64 , f64)]) -> PatternResult { let sample_count = time_samples . len () ; let mut i = 0 ; let mut buf = String :: with_capacity (NUM_BYTES) ; buf . push_str (& pattern . prefix) ; let mut n = 0 ; while i < sample_count { n += sample_pattern (pattern , & mut buf , i + 1 , sample_count) ; let dur = time_needed (& buf) ; time_samples [i] = (n as f64 , dur . as_nanos () as f64) ; if DEBUG_LEVEL >= 3 { println ! ("duration: {}" , dur . as_nanos ()) ; } if dur . as_millis () > MAX_MILLIS { return PatternResult :: TooLong ; } i += 1 ; buf . truncate (buf . len () - pattern . suffix . len ()) ; } let (score , non_linear) = SCORE_FUNCTION (time_samples) ; if DEBUG_LEVEL >= 1 { println ! ("{:<30}{:?}" , score , pattern) ; } if DEBUG_LEVEL >= 2 { println ! ("{:?}" , time_samples) ; } if non_linear { PatternResult :: NonLinear (score) } else { PatternResult :: Linear (score) } }
};
}
