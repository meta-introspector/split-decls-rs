// Generated macro for template_segment (function)
macro_rules! Depcrate_errortemplate_segment {
() => {
// Module: crate::error
// Provides: {"template_segment"}
// Dependencies: {}
fn template_segment (template_str : & str , line : usize , col : usize) -> String { let range = 3 ; let line_start = line . saturating_sub (range) ; let line_end = line + range ; let mut buf = String :: new () ; for (line_count , line_content) in template_str . lines () . enumerate () { if line_count >= line_start && line_count <= line_end { let _ = writeln ! (& mut buf , "{line_count:4} | {line_content}") ; if line_count == line - 1 { buf . push_str ("     |") ; for c in 0 .. line_content . len () { if c != col { buf . push ('-') ; } else { buf . push ('^') ; } } buf . push ('\n') ; } } } buf }
};
}
