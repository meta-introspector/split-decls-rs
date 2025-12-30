// Generated macro for OffsetIter (struct)
macro_rules! Depcrate_parseOffsetIter {
() => {
// Module: crate::parse
// Provides: {"OffsetIter"}
// Dependencies: {}
# [doc = " Markdown event and source range iterator."] # [doc = ""] # [doc = " Generates tuples where the first element is the markdown event and the second"] # [doc = " is a the corresponding range in the source string."] # [doc = ""] # [doc = " Constructed from a `Parser` using its"] # [doc = " [`into_offset_iter`](struct.Parser.html#method.into_offset_iter) method."] # [derive (Debug)] pub struct OffsetIter < 'a , F = DefaultBrokenLinkCallback > { parser : Parser < 'a , F > , }
};
}
