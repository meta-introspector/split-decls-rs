// Generated macro for EncodedCloneDescriptors (trait)
macro_rules! Depcrate_stackEncodedCloneDescriptors {
() => {
// Module: crate::stack
// Provides: {"EncodedCloneDescriptors"}
// Dependencies: {}
# [doc = " Identical role as [`EncodedDescriptors`], with the additional requirement"] # [doc = " that the block closure be [`Clone`] when implemented on [`StackBlock`]"] # [doc = " since [`StackBlock::DESCRIPTOR_WITH_CLONE`] is defined in such a context."] trait EncodedCloneDescriptors < E : ManualBlockEncoding > { const DESCRIPTOR_WITH_CLONE_AND_ENCODING : BlockDescriptorCopyDisposeSignature ; }
};
}
