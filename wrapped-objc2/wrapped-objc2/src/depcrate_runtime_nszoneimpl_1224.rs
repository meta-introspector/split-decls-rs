// Generated macro for impl_1224 (impl)
macro_rules! Depcrate_runtime_nszoneimpl_1224 {
() => {
// Module: crate::runtime::nszone
// Provides: {"impl_1224"}
// Dependencies: {}
unsafe impl RefEncode for NSZone { # [cfg (not (feature = "gnustep-1-7"))] const ENCODING_REF : Encoding = Encoding :: Pointer (& Encoding :: Struct ("_NSZone" , & [])) ; # [cfg (feature = "gnustep-1-7")] const ENCODING_REF : Encoding = Encoding :: Pointer (& Encoding :: Struct ("_NSZone" , & [Encoding :: Pointer (& Encoding :: Unknown) , Encoding :: Pointer (& Encoding :: Unknown) , Encoding :: Pointer (& Encoding :: Unknown) , Encoding :: Pointer (& Encoding :: Unknown) , Encoding :: Pointer (& Encoding :: Unknown) , Encoding :: Pointer (& Encoding :: Unknown) , Encoding :: Pointer (& Encoding :: Unknown) , usize :: ENCODING , Encoding :: Object , Encoding :: Pointer (& Encoding :: Struct ("_NSZone" , & [])) ,] ,)) ; }
};
}
