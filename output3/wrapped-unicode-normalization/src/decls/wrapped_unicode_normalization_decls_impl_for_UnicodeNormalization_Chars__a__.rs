use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> UnicodeNormalization<Chars<'a>> for &'a str {
    #[inline]
    fn nfd(self) -> Decompositions<Chars<'a>> {
        Decompositions::new_canonical(self.chars())
    }
    #[inline]
    fn nfkd(self) -> Decompositions<Chars<'a>> {
        Decompositions::new_compatible(self.chars())
    }
    #[inline]
    fn nfc(self) -> Recompositions<Chars<'a>> {
        Recompositions::new_canonical(self.chars())
    }
    #[inline]
    fn nfkc(self) -> Recompositions<Chars<'a>> {
        Recompositions::new_compatible(self.chars())
    }
    #[inline]
    fn cjk_compat_variants(self) -> Replacements<Chars<'a>> {
        Replacements::new_cjk_compat_variants(self.chars())
    }
    #[inline]
    fn stream_safe(self) -> StreamSafe<Chars<'a>> {
        StreamSafe::new(self.chars())
    }
}
