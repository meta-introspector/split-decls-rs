// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [doc = " Test a specific pattern, returning the pattern result."] # [doc = ""] # [doc = " This function prints its results to stdout."] # [doc = " No further handling is needed, the returned score is for debugging purposes mostly."] fn test (pattern : & Pattern) -> PatternResult { let mut time_samples = [(0.0 , 0.0) ; SAMPLE_SIZE] ; let mut res = PatternResult :: TooLong ; for _ in 0 .. TEST_COUNT { res = test_pattern (pattern , & mut time_samples) ; match res { PatternResult :: Linear (..) => return res , PatternResult :: NonLinear (_score) => { } PatternResult :: TooLong => { println ! ("\n\
                    possible non-linear behaviour found due to exceeding MAX_MILLIS (parsing took too long)\n\
                    pattern: {}\n\
                    score: 0\n\
                    {:?}\n" , serde_json :: to_string (& pattern) . unwrap () , time_samples ,) ; return res ; } } ; } if let PatternResult :: NonLinear (score) = res { println ! ("\n\
             possible non-linear behaviour found\n\
             pattern: {}\n\
             score: {}\n\
             {:?}\n" , serde_json :: to_string (& pattern) . unwrap () , score , time_samples ,) ; } res }
};
}
