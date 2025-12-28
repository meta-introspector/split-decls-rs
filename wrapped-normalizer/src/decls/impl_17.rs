macro_rules! deps {
    () => {
        CanonicalCompositionBorrowed!();
        Composition!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl CanonicalCompositionBorrowed < '_ > { # [doc = " Performs canonical composition (including Hangul) on a pair of"] # [doc = " characters or returns `None` if these characters don't compose."] # [doc = " Composition exclusions are taken into account."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let comp = icu::normalizer::properties::CanonicalCompositionBorrowed::new();"] # [doc = ""] # [doc = " assert_eq!(comp.compose('a', 'b'), None); // Just two non-composing starters"] # [doc = " assert_eq!(comp.compose('a', '\\u{0308}'), Some('ä'));"] # [doc = " assert_eq!(comp.compose('ẹ', '\\u{0302}'), Some('ệ'));"] # [doc = " assert_eq!(comp.compose('𝅗', '\u{1d165}'), None); // Composition exclusion"] # [doc = " assert_eq!(comp.compose('ে', '\u{9be}'), Some('ো')); // Second is starter"] # [doc = " assert_eq!(comp.compose('ᄀ', 'ᅡ'), Some('가')); // Hangul LV"] # [doc = " assert_eq!(comp.compose('가', 'ᆨ'), Some('각')); // Hangul LVT"] # [doc = " ```"] # [inline (always)] pub fn compose (self , starter : char , second : char) -> Option < char > { crate :: compose (self . canonical_compositions . canonical_compositions . iter () , starter , second ,) } }
    };
}

impl_17!();