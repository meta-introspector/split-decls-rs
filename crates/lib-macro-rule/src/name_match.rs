#[derive(Clone)]
pub enum NamedMatch {
    MatchedSeq(Box<[NamedMatch]>),
    MatchedSingle(ParseNtResult),
}
