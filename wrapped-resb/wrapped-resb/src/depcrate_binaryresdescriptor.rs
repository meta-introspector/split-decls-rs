// Generated macro for ResDescriptor (struct)
macro_rules! Depcrate_binaryResDescriptor {
() => {
// Module: crate::binary
// Provides: {"ResDescriptor"}
// Dependencies: {}
# [doc = " The `ResDescriptor` struct represents a typed pointer to a resource body"] # [doc = " within a binary resource bundle."] # [doc = ""] # [doc = " It is represented within the binary bundle as a 4-bit resource type in the"] # [doc = " most significant nibble of a 32-bit integer with a 28-bit unsigned offset"] # [doc = " in the remaining bits. The offset is interpreted as a count of 32-bit"] # [doc = " values from the start of the body."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] struct ResDescriptor { resource_type : ResourceReprType , value : u32 , }
};
}
