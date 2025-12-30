// Generated macro for BlockDescriptor (struct)
macro_rules! Depcrate_abiBlockDescriptor {
() => {
// Module: crate::abi
// Provides: {"BlockDescriptor"}
// Dependencies: {}
# [doc = " Basic block descriptor."] # [repr (C)] # [doc (alias = "__block_descriptor")] # [doc (alias = "Block_descriptor_1")] # [derive (Clone , Copy , Debug)] pub (crate) struct BlockDescriptor { # [doc = " Reserved for future use. Currently always 0."] pub (crate) reserved : c_ulong , # [doc = " Size of the block."] pub (crate) size : c_ulong , }
};
}
