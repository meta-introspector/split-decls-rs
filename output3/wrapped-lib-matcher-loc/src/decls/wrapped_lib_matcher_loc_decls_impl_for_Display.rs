use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Display for MatcherLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatcherLoc::Token { token } | MatcherLoc::SequenceSep { separator: token } => {
                write!(f, "{:?}", token)
            }
            MatcherLoc::MetaVarDecl { bind, kind, .. } => {
                write!(f, "meta-variable `${bind}:{kind}`")
            }
            MatcherLoc::Eof => f.write_str("end of macro"),
            MatcherLoc::Delimited => f.write_str("delimiter"),
            MatcherLoc::Sequence { .. } => f.write_str("sequence start"),
            MatcherLoc::SequenceKleeneOpNoSep { .. } => f.write_str("sequence end"),
            MatcherLoc::SequenceKleeneOpAfterSep { .. } => f.write_str("sequence end"),
        }
    }
}
