// Generated macro for format_case_name (function)
macro_rules! Depcrate_renderformat_case_name {
() => {
// Module: crate::render
// Provides: {"format_case_name"}
// Dependencies: {}
fn format_case_name (case : & TestCase , index : usize , display_len : usize) -> String { let description = case . description . as_ref () . map (| d | format ! ("_{d}")) . unwrap_or_default () ; format ! ("case_{index:0display_len$}{description}") }
};
}
