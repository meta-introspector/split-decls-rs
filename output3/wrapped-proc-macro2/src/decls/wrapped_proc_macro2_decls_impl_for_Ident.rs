use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Ident {
    fn _new(inner: imp::Ident) -> Self {
        Ident {
            inner,
            _marker: MARKER,
        }
    }
    fn _new_fallback(inner: fallback::Ident) -> Self {
        Ident {
            inner: imp::Ident::from(inner),
            _marker: MARKER,
        }
    }
    /// Creates a new `Ident` with the given `string` as well as the specified
    /// `span`.
    ///
    /// The `string` argument must be a valid identifier permitted by the
    /// language, otherwise the function will panic.
    ///
    /// Note that `span`, currently in rustc, configures the hygiene information
    /// for this identifier.
    ///
    /// As of this time `Span::call_site()` explicitly opts-in to "call-site"
    /// hygiene meaning that identifiers created with this span will be resolved
    /// as if they were written directly at the location of the macro call, and
    /// other code at the macro call site will be able to refer to them as well.
    ///
    /// Later spans like `Span::def_site()` will allow to opt-in to
    /// "definition-site" hygiene meaning that identifiers created with this
    /// span will be resolved at the location of the macro definition and other
    /// code at the macro call site will not be able to refer to them.
    ///
    /// Due to the current importance of hygiene this constructor, unlike other
    /// tokens, requires a `Span` to be specified at construction.
    ///
    /// # Panics
    ///
    /// Panics if the input string is neither a keyword nor a legal variable
    /// name. If you are not sure whether the string contains an identifier and
    /// need to handle an error case, use
    /// <a href="https://docs.rs/syn/2.0/syn/fn.parse_str.html"><code
    ///   style="padding-right:0;">syn::parse_str</code></a><code
    ///   style="padding-left:0;">::&lt;Ident&gt;</code>
    /// rather than `Ident::new`.
    #[track_caller]
    pub fn new(string: &str, span: Span) -> Self {
        Ident::_new(imp::Ident::new_checked(string, span.inner))
    }
    /// Same as `Ident::new`, but creates a raw identifier (`r#ident`). The
    /// `string` argument must be a valid identifier permitted by the language
    /// (including keywords, e.g. `fn`). Keywords which are usable in path
    /// segments (e.g. `self`, `super`) are not supported, and will cause a
    /// panic.
    #[track_caller]
    pub fn new_raw(string: &str, span: Span) -> Self {
        Ident::_new(imp::Ident::new_raw_checked(string, span.inner))
    }
    /// Returns the span of this `Ident`.
    pub fn span(&self) -> Span {
        Span::_new(self.inner.span())
    }
    /// Configures the span of this `Ident`, possibly changing its hygiene
    /// context.
    pub fn set_span(&mut self, span: Span) {
        self.inner.set_span(span.inner);
    }
}
