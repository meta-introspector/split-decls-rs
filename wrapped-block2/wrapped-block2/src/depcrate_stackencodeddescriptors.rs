// Generated macro for EncodedDescriptors (trait)
macro_rules! Depcrate_stackEncodedDescriptors {
() => {
// Module: crate::stack
// Provides: {"EncodedDescriptors"}
// Dependencies: {}
# [doc = " Dummy trait used in order to link [`StackBlock`]'s descriptor constants and"] # [doc = " a [`ManualBlockEncoding`] into new derived constants at compile time."] # [doc = ""] # [doc = " This is definitely a hack that should be replaced with `const fn`s defined"] # [doc = " next to [`StackBlock`]'s descriptor constants and used in the below"] # [doc = " constants' stead with inline `const`s to guarantee proper promotion."] # [doc = ""] # [doc = " See also the below [`EncodedCloneDescriptors`]."] trait EncodedDescriptors < E : ManualBlockEncoding > { const DESCRIPTOR_BASIC_WITH_ENCODING : BlockDescriptorSignature ; const DESCRIPTOR_WITH_DROP_AND_ENCODING : BlockDescriptorCopyDisposeSignature ; }
};
}
