// Generated macro for impl_28 (impl)
macro_rules! Depcrate_closerimpl_28 {
() => {
// Module: crate::closer
// Provides: {"impl_28"}
// Dependencies: {}
impl CaseMapCloser < CaseMapper > { # [doc = " A constructor which creates a [`CaseMapCloserBorrowed`] using compiled data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::casemap::CaseMapCloser;"] # [doc = " use icu::collections::codepointinvlist::CodePointInversionListBuilder;"] # [doc = ""] # [doc = " let cm = CaseMapCloser::new();"] # [doc = " let mut builder = CodePointInversionListBuilder::new();"] # [doc = " let found = cm.add_string_case_closure_to(\"ffi\", &mut builder);"] # [doc = " assert!(found);"] # [doc = " let set = builder.build();"] # [doc = ""] # [doc = " assert!(set.contains('ﬃ'));"] # [doc = ""] # [doc = " let mut builder = CodePointInversionListBuilder::new();"] # [doc = " let found = cm.add_string_case_closure_to(\"ss\", &mut builder);"] # [doc = " assert!(found);"] # [doc = " let set = builder.build();"] # [doc = ""] # [doc = " assert!(set.contains('ß'));"] # [doc = " assert!(set.contains('ẞ'));"] # [doc = " ```"] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub const fn new () -> CaseMapCloserBorrowed < 'static > { CaseMapCloserBorrowed :: new () } }
};
}
