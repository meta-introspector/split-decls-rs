// Generated macro for impl_594 (impl)
macro_rules! Depcrate_privateimpl_594 {
() => {
// Module: crate::private
// Provides: {"impl_594"}
// Dependencies: {}
# [cfg (feature = "MTLRenderPipeline")] impl MTLRenderPipelineReflection { extern_methods ! (# [cfg (feature = "MTLDevice")] # [unsafe (method (initWithVertexData : fragmentData : serializedVertexDescriptor : device : options : flags :))] pub unsafe fn initWithVertexData (this : Allocated < Self >, vertex_data : * mut c_void , fragment_data : * mut c_void , vertex_desc : * mut c_void , device : & ProtocolObject < dyn MTLDevice >, options : u64 , flags : u64 ,) -> Option < Retained < Self >>; # [unsafe (method (newSerializedVertexDataWithFlags : error : _))] pub unsafe fn newSerializedVertexDataWithFlags_error (& self , flags : u64 ,) -> Result < Retained < AnyObject >, Retained < objc2_foundation :: NSError >>; # [unsafe (method (serializeFragmentData))] pub unsafe fn serializeFragmentData (& self) -> * mut c_void ;) ; }
};
}
