use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(PartialEq, Encodable, Decodable, Debug)]
pub struct SequenceRepetition {
    /// The sequence of token trees
    pub tts: Vec<TokenTree>,
    /// The optional separator
    pub separator: Option<Token>,
    /// Whether the sequence can be repeated zero (*), or one or more times (+)
    pub kleene: KleeneToken,
    /// The number of `Match`s that appear in the sequence (and subsequences)
    pub num_captures: usize,
}
