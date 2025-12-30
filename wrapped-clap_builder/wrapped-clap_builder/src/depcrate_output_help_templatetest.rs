// Generated macro for test (module)
macro_rules! Depcrate_output_help_templatetest {
() => {
// Module: crate::output::help_template
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [test] # [cfg (feature = "wrap_help")] fn wrap_help_last_word () { use super :: * ; let help = String :: from ("foo bar baz") ; assert_eq ! (wrap (& help , 5) , "foo\nbar\nbaz") ; } # [test] # [cfg (feature = "unicode")] fn display_width_handles_non_ascii () { use super :: * ; let text = "rødgrød med fløde" ; assert_eq ! (display_width (text) , 17) ; assert_eq ! (text . len () , 20) ; } # [test] # [cfg (feature = "unicode")] fn display_width_handles_emojis () { use super :: * ; let text = "😂" ; assert_eq ! (text . chars () . count () , 1) ; assert_eq ! (display_width (text) , 2) ; assert_eq ! (text . len () , 4) ; } }
};
}
