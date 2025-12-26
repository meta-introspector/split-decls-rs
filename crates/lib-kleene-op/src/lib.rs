use rustc_macros::{Decodable, Encodable};

/// A Kleene-style [repetition operator](https://en.wikipedia.org/wiki/Kleene_star)
/// for token sequences.
#[derive(Clone, PartialEq, Encodable, Decodable, Debug, Copy)]
pub enum KleeneOp {
    /// Kleene star (`*`) for zero or more repetitions
    ZeroOrMore,
    /// Kleene plus (`+`) for one or more repetitions
    OneOrMore,
    /// Kleene optional (`?`) for zero or one repetitions
    ZeroOrOne,
}
