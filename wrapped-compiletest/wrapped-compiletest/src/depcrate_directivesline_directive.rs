// Generated macro for line_directive (function)
macro_rules! Depcrate_directivesline_directive {
() => {
// Module: crate::directives
// Provides: {"line_directive"}
// Dependencies: {}
# [doc = " If the given line begins with the appropriate comment prefix for a directive,"] # [doc = " returns a struct containing various parts of the directive."] fn line_directive < 'line > (line_number : usize , original_line : & 'line str ,) -> Option < DirectiveLine < 'line > > { let after_comment = original_line . trim_start () . strip_prefix (COMPILETEST_DIRECTIVE_PREFIX) ? . trim_start () ; let revision ; let raw_directive ; if let Some (after_open_bracket) = after_comment . strip_prefix ('[') { let Some ((line_revision , after_close_bracket)) = after_open_bracket . split_once (']') else { panic ! ("malformed condition directive: expected `{COMPILETEST_DIRECTIVE_PREFIX}[foo]`, found `{original_line}`") } ; revision = Some (line_revision) ; raw_directive = after_close_bracket . trim_start () ; } else { revision = None ; raw_directive = after_comment ; } ; Some (DirectiveLine { line_number , revision , raw_directive }) }
};
}
