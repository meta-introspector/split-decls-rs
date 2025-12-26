use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MatcherLoc {
    pub fn span(&self) -> Option<Span> {
        match self {
            MatcherLoc::Token { token } => Some(token.span),
            MatcherLoc::Delimited => None,
            MatcherLoc::Sequence { .. } => None,
            MatcherLoc::SequenceKleeneOpNoSep { .. } => None,
            MatcherLoc::SequenceSep { .. } => None,
            MatcherLoc::SequenceKleeneOpAfterSep { .. } => None,
            MatcherLoc::MetaVarDecl { span, .. } => Some(*span),
            MatcherLoc::Eof => None,
        }
    }
}
