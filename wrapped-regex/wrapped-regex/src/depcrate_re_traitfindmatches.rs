// Generated macro for FindMatches (struct)
macro_rules! Depcrate_re_traitFindMatches {
() => {
// Module: crate::re_trait
// Provides: {"FindMatches"}
// Dependencies: {}
# [doc = " An iterator over all non-overlapping successive leftmost-first matches."] pub struct FindMatches < 't , R > where R : RegularExpression , R :: Text : 't { re : R , text : & 't R :: Text , last_end : usize , last_match : Option < usize > , }
};
}
