// Generated macro for parse_bool_option (function)
macro_rules! Depcrateparse_bool_option {
() => {
// Module: crate
// Provides: {"parse_bool_option"}
// Dependencies: {}
# [doc = " Parses the same set of boolean values accepted by rustc command-line arguments."] # [doc = ""] # [doc = " Accepting all of these values is more complicated than just picking one"] # [doc = " pair, but has the advantage that contributors who are used to rustc"] # [doc = " shouldn't have to think about which values are legal."] fn parse_bool_option (value : & str) -> Option < bool > { match value { "off" | "no" | "n" | "false" => Some (false) , "on" | "yes" | "y" | "true" => Some (true) , _ => None , } }
};
}
