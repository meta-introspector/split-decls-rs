// Generated macro for detect_duplicate_flags (function)
macro_rules! Depcrate_builder_debug_assertsdetect_duplicate_flags {
() => {
// Module: crate::builder::debug_asserts
// Provides: {"detect_duplicate_flags"}
// Dependencies: {}
fn detect_duplicate_flags (flags : & [Flag < '_ >] , short_or_long : & str) { for (one , two) in find_duplicates (flags) { match (one , two) { (Flag :: Command (flag , one) , Flag :: Command (_ , another)) if one != another => panic ! ("the '{flag}' {short_or_long} flag is specified for both '{one}' and '{another}' subcommands") , (Flag :: Arg (flag , one) , Flag :: Arg (_ , another)) if one != another => panic ! ("{short_or_long} option names must be unique, but '{flag}' is in use by both '{one}' and '{another}'") , (Flag :: Arg (flag , arg) , Flag :: Command (_ , sub)) | (Flag :: Command (flag , sub) , Flag :: Arg (_ , arg)) => panic ! ("the '{flag}' {short_or_long} flag for the '{arg}' argument conflicts with the short flag \
                     for '{sub}' subcommand") , _ => { } } } }
};
}
