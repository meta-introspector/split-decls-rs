// Generated macro for LineMatchKind (enum)
macro_rules! DepcrateLineMatchKind {
() => {
// Module: crate
// Provides: {"LineMatchKind"}
// Dependencies: {}
# [doc = " The type of match for a line oriented matcher."] # [derive (Clone , Copy , Debug)] pub enum LineMatchKind { # [doc = " A position inside a line that is known to contain a match."] # [doc = ""] # [doc = " This position can be anywhere in the line. It does not need to point"] # [doc = " at the location of the match."] Confirmed (usize) , # [doc = " A position inside a line that may contain a match, and must be searched"] # [doc = " for verification."] # [doc = ""] # [doc = " This position can be anywhere in the line. It does not need to point"] # [doc = " at the location of the match."] Candidate (usize) , }
};
}
