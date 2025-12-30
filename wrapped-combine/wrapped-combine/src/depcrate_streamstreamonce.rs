// Generated macro for StreamOnce (trait)
macro_rules! Depcrate_streamStreamOnce {
() => {
// Module: crate::stream
// Provides: {"StreamOnce"}
// Dependencies: {}
# [doc = " `StreamOnce` represents a sequence of items that can be extracted one by one."] pub trait StreamOnce { # [doc = " The type of items which is yielded from this stream."] type Token : Clone ; # [doc = " The type of a range of items yielded from this stream."] # [doc = " Types which do not a have a way of yielding ranges of items should just use the"] # [doc = " `Self::Token` for this type."] type Range : Clone ; # [doc = " Type which represents the position in a stream."] # [doc = " `Ord` is required to allow parsers to determine which of two positions are further ahead."] type Position : Clone + Ord ; type Error : ParseError < Self :: Token , Self :: Range , Self :: Position > ; # [doc = " Takes a stream and removes its first token, yielding the token and the rest of the elements."] # [doc = " Returns `Err` if no element could be retrieved."] fn uncons (& mut self) -> Result < Self :: Token , StreamErrorFor < Self > > ; # [doc = " Returns `true` if this stream only contains partial input."] # [doc = ""] # [doc = " See `PartialStream`."] fn is_partial (& self) -> bool { false } }
};
}
