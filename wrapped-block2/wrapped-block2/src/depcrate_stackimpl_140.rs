// Generated macro for impl_140 (impl)
macro_rules! Depcrate_stackimpl_140 {
() => {
// Module: crate::stack
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'f , A , R , Closure , E > EncodedDescriptors < E > for StackBlock < 'f , A , R , Closure > where A : EncodeArguments , R : EncodeReturn , Closure : IntoBlock < 'f , A , R > , E : ManualBlockEncoding < Arguments = A , Return = R > , { # [doc = " [`Self::DESCRIPTOR_BASIC`] with the signature added from `E`."] const DESCRIPTOR_BASIC_WITH_ENCODING : BlockDescriptorSignature = BlockDescriptorSignature { reserved : Self :: DESCRIPTOR_BASIC . reserved , size : Self :: DESCRIPTOR_BASIC . size , encoding : E :: ENCODING_CSTR . as_ptr () , } ; # [doc = " [`Self::DESCRIPTOR_WITH_DROP`] with the signature added from `E`."] const DESCRIPTOR_WITH_DROP_AND_ENCODING : BlockDescriptorCopyDisposeSignature = BlockDescriptorCopyDisposeSignature { reserved : Self :: DESCRIPTOR_WITH_DROP . reserved , size : Self :: DESCRIPTOR_WITH_DROP . size , copy : Self :: DESCRIPTOR_WITH_DROP . copy , dispose : Self :: DESCRIPTOR_WITH_DROP . dispose , encoding : E :: ENCODING_CSTR . as_ptr () , } ; }
};
}
