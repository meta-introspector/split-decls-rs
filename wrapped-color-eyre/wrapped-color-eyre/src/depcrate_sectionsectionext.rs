// Generated macro for SectionExt (trait)
macro_rules! Depcrate_sectionSectionExt {
() => {
// Module: crate::section
// Provides: {"SectionExt"}
// Dependencies: {}
# [doc = " Extension trait for constructing sections with commonly used formats"] pub trait SectionExt : Sized { # [doc = " Add a header to a `Section` and indent the body"] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " Bodies are always indented to the same level as error messages and spans."] # [doc = " The header is not printed if the display impl of the body produces no"] # [doc = " output."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use color_eyre::{eyre::eyre, Section, SectionExt, eyre::Report};"] # [doc = ""] # [doc = " let all_in_header = \"header\\n   body\\n   body\";"] # [doc = " let report = Err::<(), Report>(eyre!(\"an error occurred\"))"] # [doc = "     .section(all_in_header)"] # [doc = "     .unwrap_err();"] # [doc = ""] # [doc = " let just_header = \"header\";"] # [doc = " let just_body = \"body\\nbody\";"] # [doc = " let report2 = Err::<(), Report>(eyre!(\"an error occurred\"))"] # [doc = "     .section(just_body.header(just_header))"] # [doc = "     .unwrap_err();"] # [doc = ""] # [doc = " assert_eq!(format!(\"{:?}\", report), format!(\"{:?}\", report2))"] # [doc = " ```"] fn header < C > (self , header : C) -> IndentedSection < C , Self > where C : Display + Send + Sync + 'static ; }
};
}
