// Generated macro for escape (function)
macro_rules! Depcrateescape {
() => {
// Module: crate
// Provides: {"escape"}
// Dependencies: {}
# [doc = " Escape meta-characters within the given glob pattern."] # [doc = ""] # [doc = " The escaping works by surrounding meta-characters with brackets. For"] # [doc = " example, `*` becomes `[*]`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use globset::escape;"] # [doc = ""] # [doc = " assert_eq!(escape(\"foo*bar\"), \"foo[*]bar\");"] # [doc = " assert_eq!(escape(\"foo?bar\"), \"foo[?]bar\");"] # [doc = " assert_eq!(escape(\"foo[bar\"), \"foo[[]bar\");"] # [doc = " assert_eq!(escape(\"foo]bar\"), \"foo[]]bar\");"] # [doc = " assert_eq!(escape(\"foo{bar\"), \"foo[{]bar\");"] # [doc = " assert_eq!(escape(\"foo}bar\"), \"foo[}]bar\");"] # [doc = " ```"] pub fn escape (s : & str) -> String { let mut escaped = String :: with_capacity (s . len ()) ; for c in s . chars () { match c { '?' | '*' | '[' | ']' | '{' | '}' => { escaped . push ('[') ; escaped . push (c) ; escaped . push (']') ; } c => { escaped . push (c) ; } } } escaped }
};
}
