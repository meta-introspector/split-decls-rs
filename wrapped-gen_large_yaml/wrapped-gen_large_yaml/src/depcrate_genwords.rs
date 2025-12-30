// Generated macro for words (function)
macro_rules! Depcrate_genwords {
() => {
// Module: crate::gen
// Provides: {"words"}
// Dependencies: {}
# [doc = " Generate a set of words."] pub fn words (rng : & mut SmallRng , words_lo : usize , words_hi : usize) -> String { let nwords = rng . gen_range (words_lo .. words_hi) ; lipsum :: lipsum_words_with_rng (rng . clone () , nwords) . replace (| c | "-\'\",*:" . contains (c) , "") }
};
}
