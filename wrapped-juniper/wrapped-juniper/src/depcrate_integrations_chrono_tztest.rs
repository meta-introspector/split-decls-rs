// Generated macro for test (module)
macro_rules! Depcrate_integrations_chrono_tztest {
() => {
// Module: crate::integrations::chrono_tz
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: TimeZone ; mod from_input_value { use super :: TimeZone ; use crate :: { FromInputValue , InputValue , IntoFieldError , graphql } ; fn tz_input_test (raw : & 'static str , expected : Result < TimeZone , & str >) { let input : InputValue = graphql :: input_value ! ((raw)) ; let parsed = FromInputValue :: from_input_value (& input) ; assert_eq ! (parsed . as_ref () , expected . map_err (IntoFieldError :: into_field_error) . as_ref () ,) ; } # [test] fn europe_zone () { tz_input_test ("Europe/London" , Ok (chrono_tz :: Europe :: London)) ; } # [test] fn etc_minus () { tz_input_test ("Etc/GMT-3" , Ok (chrono_tz :: Etc :: GMTMinus3)) ; } mod invalid { use super :: tz_input_test ; # [test] fn forward_slash () { tz_input_test ("Abc/Xyz" , Err ("Failed to parse `TimeZone`: failed to parse timezone") ,) ; } # [test] fn number () { tz_input_test ("8086" , Err ("Failed to parse `TimeZone`: failed to parse timezone") ,) ; } # [test] fn no_forward_slash () { tz_input_test ("AbcXyz" , Err ("Failed to parse `TimeZone`: failed to parse timezone") ,) ; } } } }
};
}
