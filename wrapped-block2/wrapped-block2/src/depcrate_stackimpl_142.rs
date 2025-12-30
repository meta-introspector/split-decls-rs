// Generated macro for impl_142 (impl)
macro_rules! Depcrate_stackimpl_142 {
() => {
// Module: crate::stack
// Provides: {"impl_142"}
// Dependencies: {}
impl < 'f , A , R , Closure , E > EncodedCloneDescriptors < E > for StackBlock < 'f , A , R , Closure > where A : EncodeArguments , R : EncodeReturn , Closure : IntoBlock < 'f , A , R > + Clone , E : ManualBlockEncoding < Arguments = A , Return = R > , { # [doc = " [`Self::DESCRIPTOR_WITH_CLONE`] with the signature added from `E`."] const DESCRIPTOR_WITH_CLONE_AND_ENCODING : BlockDescriptorCopyDisposeSignature = BlockDescriptorCopyDisposeSignature { reserved : Self :: DESCRIPTOR_WITH_CLONE . reserved , size : Self :: DESCRIPTOR_WITH_CLONE . size , copy : Self :: DESCRIPTOR_WITH_CLONE . copy , dispose : Self :: DESCRIPTOR_WITH_CLONE . dispose , encoding : E :: ENCODING_CSTR . as_ptr () , } ; }
};
}
