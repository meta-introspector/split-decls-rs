// Generated macro for MTLResourceID (struct)
macro_rules! Depcrate_typesMTLResourceID {
() => {
// Module: crate::types
// Provides: {"MTLResourceID"}
// Dependencies: {}
# [doc = " Handle of the GPU resource used for binding resources to argument tables,"] # [doc = " navigating resource view pools and storing resources in an argument buffer"] # [doc = ""] # [doc = " MTLResourceID represents a specific GPU resource. This handle can be"] # [doc = " mutated by modifying textureID or samplerID values to get to individual"] # [doc = " resource views in a resource view pool."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlresourceid?language=objc)"] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq)] pub struct MTLResourceID { pub (crate) _impl : u64 , }
};
}
