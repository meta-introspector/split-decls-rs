// Generated macro for ArangeHeaderIter (struct)
macro_rules! Depcrate_read_arangesArangeHeaderIter {
() => {
// Module: crate::read::aranges
// Provides: {"ArangeHeaderIter"}
// Dependencies: {}
# [doc = " An iterator over the headers of a `.debug_aranges` section."] # [derive (Clone , Debug)] pub struct ArangeHeaderIter < R : Reader > { input : R , offset : DebugArangesOffset < R :: Offset > , }
};
}
