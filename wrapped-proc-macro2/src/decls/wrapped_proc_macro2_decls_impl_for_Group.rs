use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Group {
    fn _new(inner: imp::Group) -> Self {
        Group { inner }
    }
    fn _new_fallback(inner: fallback::Group) -> Self {
        Group {
            inner: imp::Group::from(inner),
        }
    }
    /// Creates a new `Group` with the given delimiter and token stream.
    ///
    /// This constructor will set the span for this group to
    /// `Span::call_site()`. To change the span you can use the `set_span`
    /// method below.
    pub fn new(delimiter: Delimiter, stream: TokenStream) -> Self {
        Group {
            inner: imp::Group::new(delimiter, stream.inner),
        }
    }
    /// Returns the punctuation used as the delimiter for this group: a set of
    /// parentheses, square brackets, or curly braces.
    pub fn delimiter(&self) -> Delimiter {
        self.inner.delimiter()
    }
    /// Returns the `TokenStream` of tokens that are delimited in this `Group`.
    ///
    /// Note that the returned token stream does not include the delimiter
    /// returned above.
    pub fn stream(&self) -> TokenStream {
        TokenStream::_new(self.inner.stream())
    }
    /// Returns the span for the delimiters of this token stream, spanning the
    /// entire `Group`.
    ///
    /// ```text
    /// pub fn span(&self) -> Span {
    ///            ^^^^^^^
    /// ```
    pub fn span(&self) -> Span {
        Span::_new(self.inner.span())
    }
    /// Returns the span pointing to the opening delimiter of this group.
    ///
    /// ```text
    /// pub fn span_open(&self) -> Span {
    ///                 ^
    /// ```
    pub fn span_open(&self) -> Span {
        Span::_new(self.inner.span_open())
    }
    /// Returns the span pointing to the closing delimiter of this group.
    ///
    /// ```text
    /// pub fn span_close(&self) -> Span {
    ///                        ^
    /// ```
    pub fn span_close(&self) -> Span {
        Span::_new(self.inner.span_close())
    }
    /// Returns an object that holds this group's `span_open()` and
    /// `span_close()` together (in a more compact representation than holding
    /// those 2 spans individually).
    pub fn delim_span(&self) -> DelimSpan {
        DelimSpan::new(&self.inner)
    }
    /// Configures the span for this `Group`'s delimiters, but not its internal
    /// tokens.
    ///
    /// This method will **not** set the span of all the internal tokens spanned
    /// by this group, but rather it will only set the span of the delimiter
    /// tokens at the level of the `Group`.
    pub fn set_span(&mut self, span: Span) {
        self.inner.set_span(span.inner);
    }
}
