use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Punct {
    /// Creates a new `Punct` from the given character and spacing.
    ///
    /// The `ch` argument must be a valid punctuation character permitted by the
    /// language, otherwise the function will panic.
    ///
    /// The returned `Punct` will have the default span of `Span::call_site()`
    /// which can be further configured with the `set_span` method below.
    pub fn new(ch: char, spacing: Spacing) -> Self {
        if let '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+' | ',' | '-' | '.' | '/'
        | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '^' | '|' | '~' = ch {
            Punct {
                ch,
                spacing,
                span: Span::call_site(),
            }
        } else {
            panic!("unsupported proc macro punctuation character {:?}", ch);
        }
    }
    /// Returns the value of this punctuation character as `char`.
    pub fn as_char(&self) -> char {
        self.ch
    }
    /// Returns the spacing of this punctuation character, indicating whether
    /// it's immediately followed by another `Punct` in the token stream, so
    /// they can potentially be combined into a multicharacter operator
    /// (`Joint`), or it's followed by some other token or whitespace (`Alone`)
    /// so the operator has certainly ended.
    pub fn spacing(&self) -> Spacing {
        self.spacing
    }
    /// Returns the span for this punctuation character.
    pub fn span(&self) -> Span {
        self.span
    }
    /// Configure the span for this punctuation character.
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
