// Generated macro for KleeneOp (enum)
macro_rules! DepcrateKleeneOp {
() => {
// Module: crate
// Provides: {"KleeneOp"}
// Dependencies: {}
# [doc = " A Kleene-style [repetition operator](https://en.wikipedia.org/wiki/Kleene_star)"] # [doc = " for token sequences."] # [derive (Clone , PartialEq , Encodable , Decodable , Debug , Copy)] pub enum KleeneOp { # [doc = " Kleene star (`*`) for zero or more repetitions"] ZeroOrMore , # [doc = " Kleene plus (`+`) for one or more repetitions"] OneOrMore , # [doc = " Kleene optional (`?`) for zero or one repetitions"] ZeroOrOne , }
};
}
