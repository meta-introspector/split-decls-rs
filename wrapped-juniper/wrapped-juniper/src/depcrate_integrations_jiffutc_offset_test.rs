// Generated macro for utc_offset_test (module)
macro_rules! Depcrate_integrations_jiffutc_offset_test {
() => {
// Module: crate::integrations::jiff
// Provides: {"utc_offset_test"}
// Dependencies: {}
# [cfg (test)] mod utc_offset_test { use jiff :: tz ; use crate :: { FromInputValue as _ , InputValue , ToInputValue as _ , graphql } ; use super :: UtcOffset ; # [test] fn parses_correct_input () { for (raw , expected) in [("+00:00" , tz :: offset (0)) , ("+03:00" , tz :: offset (3)) , ("-09:00" , tz :: offset (- 9)) ,] { let input : InputValue = graphql :: input_value ! ((raw)) ; let parsed = UtcOffset :: from_input_value (& input) ; assert ! (parsed . is_ok () , "failed to parse `{raw}`: {:?}" , parsed . unwrap_err () ,) ; assert_eq ! (parsed . unwrap () , expected , "input: {raw}") ; } } # [test] fn fails_on_invalid_input () { for input in [graphql :: input_value ! ("Europe/London") , graphql :: input_value ! ("Abc/Xyz") , graphql :: input_value ! ("8086") , graphql :: input_value ! ("AbcXyz") , graphql :: input_value ! ("Z") , graphql :: input_value ! ("i'm not even a time zone") , graphql :: input_value ! (2.32) , graphql :: input_value ! (1) , graphql :: input_value ! (null) , graphql :: input_value ! (false) ,] { let input : InputValue = input ; let parsed = UtcOffset :: from_input_value (& input) ; assert ! (parsed . is_err () , "allows input: {input:?}") ; } } # [test] fn formats_correctly () { for (val , expected) in [(tz :: offset (0) , graphql :: input_value ! ("+00:00")) , (tz :: offset (2) , graphql :: input_value ! ("+02:00")) , (tz :: offset (- 11) , graphql :: input_value ! ("-11:00")) ,] { let actual : InputValue = val . to_input_value () ; assert_eq ! (actual , expected , "on value: {val:?}") ; } } }
};
}
