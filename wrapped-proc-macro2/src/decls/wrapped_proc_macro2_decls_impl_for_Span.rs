use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Span {
    fn _new(inner: imp::Span) -> Self {
        Span {
            inner,
            _marker: MARKER,
        }
    }
    fn _new_fallback(inner: fallback::Span) -> Self {
        Span {
            inner: imp::Span::from(inner),
            _marker: MARKER,
        }
    }
    /// The span of the invocation of the current procedural macro.
    ///
    /// Identifiers created with this span will be resolved as if they were
    /// written directly at the macro call location (call-site hygiene) and
    /// other code at the macro call site will be able to refer to them as well.
    pub fn call_site() -> Self {
        Span::_new(imp::Span::call_site())
    }
    /// The span located at the invocation of the procedural macro, but with
    /// local variables, labels, and `$crate` resolved at the definition site
    /// of the macro. This is the same hygiene behavior as `macro_rules`.
    pub fn mixed_site() -> Self {
        Span::_new(imp::Span::mixed_site())
    }
    /// A span that resolves at the macro definition site.
    ///
    /// This method is semver exempt and not exposed by default.
    #[cfg(procmacro2_semver_exempt)]
    #[cfg_attr(docsrs, doc(cfg(procmacro2_semver_exempt)))]
    pub fn def_site() -> Self {
        Span::_new(imp::Span::def_site())
    }
    /// Creates a new span with the same line/column information as `self` but
    /// that resolves symbols as though it were at `other`.
    pub fn resolved_at(&self, other: Span) -> Span {
        Span::_new(self.inner.resolved_at(other.inner))
    }
    /// Creates a new span with the same name resolution behavior as `self` but
    /// with the line/column information of `other`.
    pub fn located_at(&self, other: Span) -> Span {
        Span::_new(self.inner.located_at(other.inner))
    }
    /// Convert `proc_macro2::Span` to `proc_macro::Span`.
    ///
    /// This method is available when building with a nightly compiler, or when
    /// building with rustc 1.29+ *without* semver exempt features.
    ///
    /// # Panics
    ///
    /// Panics if called from outside of a procedural macro. Unlike
    /// `proc_macro2::Span`, the `proc_macro::Span` type can only exist within
    /// the context of a procedural macro invocation.
    #[cfg(wrap_proc_macro)]
    pub fn unwrap(self) -> proc_macro::Span {
        self.inner.unwrap()
    }
    #[cfg(wrap_proc_macro)]
    #[doc(hidden)]
    pub fn unstable(self) -> proc_macro::Span {
        self.unwrap()
    }
    /// Returns the span's byte position range in the source file.
    ///
    /// This method requires the `"span-locations"` feature to be enabled.
    ///
    /// When executing in a procedural macro context, the returned range is only
    /// accurate if compiled with a nightly toolchain. The stable toolchain does
    /// not have this information available. When executing outside of a
    /// procedural macro, such as main.rs or build.rs, the byte range is always
    /// accurate regardless of toolchain.
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn byte_range(&self) -> Range<usize> {
        self.inner.byte_range()
    }
    /// Get the starting line/column in the source file for this span.
    ///
    /// This method requires the `"span-locations"` feature to be enabled.
    ///
    /// When executing in a procedural macro context, the returned line/column
    /// are only meaningful if compiled with a nightly toolchain. The stable
    /// toolchain does not have this information available. When executing
    /// outside of a procedural macro, such as main.rs or build.rs, the
    /// line/column are always meaningful regardless of toolchain.
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn start(&self) -> LineColumn {
        self.inner.start()
    }
    /// Get the ending line/column in the source file for this span.
    ///
    /// This method requires the `"span-locations"` feature to be enabled.
    ///
    /// When executing in a procedural macro context, the returned line/column
    /// are only meaningful if compiled with a nightly toolchain. The stable
    /// toolchain does not have this information available. When executing
    /// outside of a procedural macro, such as main.rs or build.rs, the
    /// line/column are always meaningful regardless of toolchain.
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn end(&self) -> LineColumn {
        self.inner.end()
    }
    /// The path to the source file in which this span occurs, for display
    /// purposes.
    ///
    /// This might not correspond to a valid file system path. It might be
    /// remapped, or might be an artificial path such as `"<macro expansion>"`.
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn file(&self) -> String {
        self.inner.file()
    }
    /// The path to the source file in which this span occurs on disk.
    ///
    /// This is the actual path on disk. It is unaffected by path remapping.
    ///
    /// This path should not be embedded in the output of the macro; prefer
    /// `file()` instead.
    #[cfg(span_locations)]
    #[cfg_attr(docsrs, doc(cfg(feature = "span-locations")))]
    pub fn local_file(&self) -> Option<PathBuf> {
        self.inner.local_file()
    }
    /// Create a new span encompassing `self` and `other`.
    ///
    /// Returns `None` if `self` and `other` are from different files.
    ///
    /// Warning: the underlying [`proc_macro::Span::join`] method is
    /// nightly-only. When called from within a procedural macro not using a
    /// nightly compiler, this method will always return `None`.
    pub fn join(&self, other: Span) -> Option<Span> {
        self.inner.join(other.inner).map(Span::_new)
    }
    /// Compares two spans to see if they're equal.
    ///
    /// This method is semver exempt and not exposed by default.
    #[cfg(procmacro2_semver_exempt)]
    #[cfg_attr(docsrs, doc(cfg(procmacro2_semver_exempt)))]
    pub fn eq(&self, other: &Span) -> bool {
        self.inner.eq(&other.inner)
    }
    /// Returns the source text behind a span. This preserves the original
    /// source code, including spaces and comments. It only returns a result if
    /// the span corresponds to real source code.
    ///
    /// Note: The observable result of a macro should only rely on the tokens
    /// and not on this source text. The result of this function is a best
    /// effort to be used for diagnostics only.
    pub fn source_text(&self) -> Option<String> {
        self.inner.source_text()
    }
}
