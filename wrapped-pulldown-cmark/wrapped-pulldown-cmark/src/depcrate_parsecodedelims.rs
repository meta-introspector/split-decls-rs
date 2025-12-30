// Generated macro for CodeDelims (struct)
macro_rules! Depcrate_parseCodeDelims {
() => {
// Module: crate::parse
// Provides: {"CodeDelims"}
// Dependencies: {}
# [doc = " Tracks tree indices of code span delimiters of each length. It should prevent"] # [doc = " quadratic scanning behaviours by providing (amortized) constant time lookups."] struct CodeDelims { inner : HashMap < usize , VecDeque < TreeIndex > > , seen_first : bool , }
};
}
