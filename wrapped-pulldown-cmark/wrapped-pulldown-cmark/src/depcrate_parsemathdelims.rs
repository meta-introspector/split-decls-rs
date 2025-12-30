// Generated macro for MathDelims (struct)
macro_rules! Depcrate_parseMathDelims {
() => {
// Module: crate::parse
// Provides: {"MathDelims"}
// Dependencies: {}
# [doc = " Tracks brace contexts and delimiter length for math delimiters."] # [doc = " Provides amortized constant-time lookups."] struct MathDelims { inner : HashMap < u8 , VecDeque < (TreeIndex , bool , bool) > > , }
};
}
