// Generated macro for BlockHeader (struct)
macro_rules! Depcrate_xzBlockHeader {
() => {
// Module: crate::xz
// Provides: {"BlockHeader"}
// Dependencies: {}
# [derive (Debug)] struct BlockHeader { header_size : usize , compressed_size : Option < u64 > , uncompressed_size : Option < u64 > , filters : [Option < FilterType > ; 4] , properties : [u32 ; 4] , }
};
}
