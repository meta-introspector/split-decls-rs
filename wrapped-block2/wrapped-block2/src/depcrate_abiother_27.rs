// Generated macro for other_27 (other)
macro_rules! Depcrate_abiother_27 {
() => {
// Module: crate::abi
// Provides: {"other_27"}
// Dependencies: {}
# [doc = " The type of this is:"] # [doc = " ```pseudo-code"] # [doc = " match (BLOCK_HAS_COPY_DISPOSE, BLOCK_HAS_SIGNATURE) {"] # [doc = "     (false, false) => BlockDescriptor,"] # [doc = "     (true, false) => BlockDescriptorCopyDispose,"] # [doc = "     (false, true) => BlockDescriptorSignature,"] # [doc = "     (true, true) => BlockDescriptorCopyDisposeSignature,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Since all of these start with `BlockDescriptor`, it is always safe to"] # [doc = " use the `basic` field."] # [repr (C)] # [derive (Clone , Copy)] pub (crate) union BlockDescriptorPtr { pub (crate) basic : * const BlockDescriptor , pub (crate) with_copy_dispose : * const BlockDescriptorCopyDispose , pub (crate) with_signature : * const BlockDescriptorSignature , pub (crate) with_copy_dispose_signature : * const BlockDescriptorCopyDisposeSignature , }
};
}
