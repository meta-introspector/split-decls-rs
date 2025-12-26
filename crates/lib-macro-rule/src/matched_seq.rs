#[derive(Clone)] // Add derive(Clone) here
pub enum MatchedSeq {
    // This enum needs to represent sequence of matches. Since its just a wrapper around a Vec of NamedMatch,
    // It's probably better to just use MatchedSeq(Box<[NamedMatch]>) directly, as defined in NamedMatch.
    // If there were other variants, they would be here. For now, it's just a placeholder.
}
