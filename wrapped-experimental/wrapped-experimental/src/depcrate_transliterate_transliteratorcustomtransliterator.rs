// Generated macro for CustomTransliterator (trait)
macro_rules! Depcrate_transliterate_transliteratorCustomTransliterator {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"CustomTransliterator"}
// Dependencies: {}
# [doc = " A type that supports transliteration. Used for overrides in [`Transliterator`] - see"] # [doc = " [`Transliterator::try_new_with_override_unstable`]."] pub trait CustomTransliterator : Debug { # [doc = " Transliterates the portion of the input string specified by the byte indices in the range."] # [doc = ""] # [doc = " The returned `String` must just be the transliteration of `input[range]`. The rest is"] # [doc = " there for context, if necessary."] fn transliterate (& self , input : & str , range : Range < usize >) -> String ; }
};
}
