use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Methods for iterating over strings while applying Unicode normalizations
/// as described in
/// [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).
pub trait UnicodeNormalization<I: Iterator<Item = char>> {
    /// Returns an iterator over the string in Unicode Normalization Form D
    /// (canonical decomposition).
    fn nfd(self) -> Decompositions<I>;
    /// Returns an iterator over the string in Unicode Normalization Form KD
    /// (compatibility decomposition).
    fn nfkd(self) -> Decompositions<I>;
    /// An Iterator over the string in Unicode Normalization Form C
    /// (canonical decomposition followed by canonical composition).
    fn nfc(self) -> Recompositions<I>;
    /// An Iterator over the string in Unicode Normalization Form KC
    /// (compatibility decomposition followed by canonical composition).
    fn nfkc(self) -> Recompositions<I>;
    /// A transformation which replaces [CJK Compatibility Ideograph] codepoints
    /// with normal forms using [Standardized Variation Sequences]. This is not
    /// part of the canonical or compatibility decomposition algorithms, but
    /// performing it before those algorithms produces normalized output which
    /// better preserves the intent of the original text.
    ///
    /// Note that many systems today ignore variation selectors, so these
    /// may not immediately help text display as intended, but they at
    /// least preserve the information in a standardized form, giving
    /// implementations the option to recognize them.
    ///
    /// [CJK Compatibility Ideograph]: https://www.unicode.org/glossary/#compatibility_ideograph
    /// [Standardized Variation Sequences]: https://www.unicode.org/glossary/#standardized_variation_sequence
    fn cjk_compat_variants(self) -> Replacements<I>;
    /// An Iterator over the string with Conjoining Grapheme Joiner characters
    /// inserted according to the Stream-Safe Text Process ([UAX15-D4]).
    ///
    /// [UAX15-D4]: https://www.unicode.org/reports/tr15/#UAX15-D4
    fn stream_safe(self) -> StreamSafe<I>;
}
