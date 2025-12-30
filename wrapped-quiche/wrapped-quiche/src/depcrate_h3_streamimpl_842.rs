// Generated macro for impl_842 (impl)
macro_rules! Depcrate_h3_streamimpl_842 {
() => {
// Module: crate::h3::stream
// Provides: {"impl_842"}
// Dependencies: {}
impl Type { # [cfg (feature = "qlog")] pub fn to_qlog (self) -> qlog :: events :: h3 :: H3StreamType { match self { Type :: Control => qlog :: events :: h3 :: H3StreamType :: Control , Type :: Request => qlog :: events :: h3 :: H3StreamType :: Request , Type :: Push => qlog :: events :: h3 :: H3StreamType :: Push , Type :: QpackEncoder => qlog :: events :: h3 :: H3StreamType :: QpackEncode , Type :: QpackDecoder => qlog :: events :: h3 :: H3StreamType :: QpackDecode , Type :: Unknown => qlog :: events :: h3 :: H3StreamType :: Unknown , } } }
};
}
