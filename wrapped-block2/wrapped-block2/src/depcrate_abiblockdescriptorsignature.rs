// Generated macro for BlockDescriptorSignature (struct)
macro_rules! Depcrate_abiBlockDescriptorSignature {
() => {
// Module: crate::abi
// Provides: {"BlockDescriptorSignature"}
// Dependencies: {}
# [doc = " Block descriptor that has an encoding / a signature."] # [doc = ""] # [doc = " Requires BLOCK_HAS_SIGNATURE."] # [repr (C)] # [doc (alias = "__block_descriptor")] # [doc (alias = "Block_descriptor_3")] # [derive (Clone , Copy , Debug)] pub (crate) struct BlockDescriptorSignature { # [doc = " Reserved for future use. Currently always 0."] pub (crate) reserved : c_ulong , # [doc = " Size of the block."] pub (crate) size : c_ulong , # [doc = " Objective-C type encoding of the block."] # [doc (alias = "signature")] pub (crate) encoding : * const c_char , }
};
}
