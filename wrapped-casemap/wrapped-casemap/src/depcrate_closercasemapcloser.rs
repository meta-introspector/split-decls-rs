// Generated macro for CaseMapCloser (struct)
macro_rules! Depcrate_closerCaseMapCloser {
() => {
// Module: crate::closer
// Provides: {"CaseMapCloser"}
// Dependencies: {}
# [doc = " A wrapper around [`CaseMapper`] that can produce case mapping closures"] # [doc = " over a character or string. This wrapper can be constructed directly, or"] # [doc = " by wrapping a reference to an existing [`CaseMapper`]."] # [doc = ""] # [doc = " Most methods for this type live on [`CaseMapCloserBorrowed`], which you can obtain via"] # [doc = " [`CaseMapCloser::new()`] or [`CaseMapCloser::as_borrowed()`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::casemap::CaseMapCloser;"] # [doc = " use icu::collections::codepointinvlist::CodePointInversionListBuilder;"] # [doc = ""] # [doc = " let cm = CaseMapCloser::new();"] # [doc = " let mut builder = CodePointInversionListBuilder::new();"] # [doc = " let found = cm.add_string_case_closure_to(\"ffi\", &mut builder);"] # [doc = " assert!(found);"] # [doc = " let set = builder.build();"] # [doc = ""] # [doc = " assert!(set.contains('ﬃ'));"] # [doc = ""] # [doc = " let mut builder = CodePointInversionListBuilder::new();"] # [doc = " let found = cm.add_string_case_closure_to(\"ss\", &mut builder);"] # [doc = " assert!(found);"] # [doc = " let set = builder.build();"] # [doc = ""] # [doc = " assert!(set.contains('ß'));"] # [doc = " assert!(set.contains('ẞ'));"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct CaseMapCloser < CM > { cm : CM , unfold : DataPayload < CaseMapUnfoldV1 > , }
};
}
