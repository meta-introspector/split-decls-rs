// Generated macro for parse_normalize_rule (function)
macro_rules! Depcrate_directivesparse_normalize_rule {
() => {
// Module: crate::directives
// Provides: {"parse_normalize_rule"}
// Dependencies: {}
# [doc = " Parses the regex and replacement values of a `//@ normalize-*` directive, in the format:"] # [doc = " ```text"] # [doc = " \"REGEX\" -> \"REPLACEMENT\""] # [doc = " ```"] fn parse_normalize_rule (raw_value : & str) -> Option < (String , String) > { let captures = static_regex ! (r#"(?x) # (verbose mode regex)
        ^
        \s*                     # (leading whitespace)
        "(?<regex>[^"]*)"       # "REGEX"
        \s+->\s+                # ->
        "(?<replacement>[^"]*)" # "REPLACEMENT"
        $
        "#) . captures (raw_value) ? ; let regex = captures ["regex"] . to_owned () ; let replacement = captures ["replacement"] . to_owned () ; let replacement = replacement . replace ("\\n" , "\n") ; Some ((regex , replacement)) }
};
}
