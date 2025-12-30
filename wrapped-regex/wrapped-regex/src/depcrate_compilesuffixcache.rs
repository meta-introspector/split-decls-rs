// Generated macro for SuffixCache (struct)
macro_rules! Depcrate_compileSuffixCache {
() => {
// Module: crate::compile
// Provides: {"SuffixCache"}
// Dependencies: {}
# [doc = " SuffixCache is a simple bounded hash map for caching suffix entries in"] # [doc = " UTF-8 automata. For example, consider the Unicode range \\u{0}-\\u{FFFF}."] # [doc = " The set of byte ranges looks like this:"] # [doc = ""] # [doc = " [0-7F]"] # [doc = " [C2-DF][80-BF]"] # [doc = " [E0][A0-BF][80-BF]"] # [doc = " [E1-EC][80-BF][80-BF]"] # [doc = " [ED][80-9F][80-BF]"] # [doc = " [EE-EF][80-BF][80-BF]"] # [doc = ""] # [doc = " Each line above translates to one alternate in the compiled regex program."] # [doc = " However, all but one of the alternates end in the same suffix, which is"] # [doc = " a waste of an instruction. The suffix cache facilitates reusing them across"] # [doc = " alternates."] # [doc = ""] # [doc = " Note that a HashMap could be trivially used for this, but we don't need its"] # [doc = " overhead. Some small bounded space (LRU style) is more than enough."] struct SuffixCache { table : Vec < SuffixCacheEntry > , version : usize , }
};
}
