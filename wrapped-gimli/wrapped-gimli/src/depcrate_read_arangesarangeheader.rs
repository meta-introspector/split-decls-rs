// Generated macro for ArangeHeader (struct)
macro_rules! Depcrate_read_arangesArangeHeader {
() => {
// Module: crate::read::aranges
// Provides: {"ArangeHeader"}
// Dependencies: {}
# [doc = " A header for a set of entries in the `.debug_arange` section."] # [doc = ""] # [doc = " These entries all belong to a single unit."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct ArangeHeader < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { offset : DebugArangesOffset < Offset > , encoding : Encoding , length : Offset , debug_info_offset : DebugInfoOffset < Offset > , entries : R , }
};
}
