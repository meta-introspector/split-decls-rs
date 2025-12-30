// Generated macro for BlockDescriptorCopyDisposeSignature (struct)
macro_rules! Depcrate_abiBlockDescriptorCopyDisposeSignature {
() => {
// Module: crate::abi
// Provides: {"BlockDescriptorCopyDisposeSignature"}
// Dependencies: {}
# [doc = " Block descriptor that contains copy and dispose operations, and which"] # [doc = " has an encoding / a signature."] # [doc = ""] # [doc = " Requires BLOCK_HAS_COPY_DISPOSE and BLOCK_HAS_SIGNATURE."] # [repr (C)] # [doc (alias = "__block_descriptor")] # [doc (alias = "Block_descriptor_2")] # [doc (alias = "Block_descriptor_3")] # [derive (Clone , Copy , Debug)] pub (crate) struct BlockDescriptorCopyDisposeSignature { # [doc = " Reserved for future use. Currently always 0."] pub (crate) reserved : c_ulong , # [doc = " Size of the block."] pub (crate) size : c_ulong , # [doc = " Helper to copy the block if it contains nontrivial copy operations."] # [doc = ""] # [doc = " This may be NULL since macOS 11.0.1 in Apple's runtime, but this"] # [doc = " should not be relied on."] pub (crate) copy : Option < unsafe extern "C-unwind" fn (dst : * mut c_void , src : * const c_void) > , # [doc = " Helper to destroy the block after being copied."] # [doc = ""] # [doc = " This may be NULL since macOS 11.0.1 in Apple's runtime, but this"] # [doc = " should not be relied on."] pub (crate) dispose : Option < unsafe extern "C-unwind" fn (src : * mut c_void) > , # [doc = " Objective-C type encoding of the block."] # [doc (alias = "signature")] pub (crate) encoding : * const c_char , }
};
}
