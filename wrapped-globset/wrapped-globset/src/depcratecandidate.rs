// Generated macro for Candidate (struct)
macro_rules! DepcrateCandidate {
() => {
// Module: crate
// Provides: {"Candidate"}
// Dependencies: {}
# [doc = " A candidate path for matching."] # [doc = ""] # [doc = " All glob matching in this crate operates on `Candidate` values."] # [doc = " Constructing candidates has a very small cost associated with it, so"] # [doc = " callers may find it beneficial to amortize that cost when matching a single"] # [doc = " path against multiple globs or sets of globs."] # [derive (Clone)] pub struct Candidate < 'a > { path : Cow < 'a , [u8] > , basename : Cow < 'a , [u8] > , ext : Cow < 'a , [u8] > , }
};
}
