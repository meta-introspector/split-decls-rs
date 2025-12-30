// Generated macro for CE_BUFFER_SIZE (const)
macro_rules! Depcrate_elementsCE_BUFFER_SIZE {
() => {
// Module: crate::elements
// Provides: {"CE_BUFFER_SIZE"}
// Dependencies: {}
# [doc = " The number of full 64-bit collation units that get buffered"] # [doc = " in the primary comparison loop so that they can be examined"] # [doc = " by the subsequent comparison stregths."] # [doc = ""] # [doc = " Note 1: If a primary difference is found, the comparison"] # [doc = " returns early, so these buffers end up holding all the"] # [doc = " collation elements only if there is no primary difference."] # [doc = ""] # [doc = " Note 2: Unfortunately for now, a sentinel value signaling"] # [doc = " the end of input gets written into the buffer in addition"] # [doc = " to the real collation elements."] # [doc = ""] # [doc = " This should probably either be halved to 4 on the logic"] # [doc = " that especially in the presence of the identical prefix"] # [doc = " optimization, most comparisons return after a couple of"] # [doc = " primary comparisons or increased to 32 on the logic that"] # [doc = " such a buffer could better hold a file or human name that"] # [doc = " differs on secordary or higher level."] pub (crate) const CE_BUFFER_SIZE : usize = 8 ;
};
}
