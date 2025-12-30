// Generated macro for MemoryAttributesTable (struct)
macro_rules! Depcrate_systemMemoryAttributesTable {
() => {
// Module: crate::system
// Provides: {"MemoryAttributesTable"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct MemoryAttributesTable < const N : usize = 0 > { pub version : u32 , pub number_of_entries : u32 , pub descriptor_size : u32 , pub reserved : u32 , pub entry : [MemoryDescriptor ; N] , }
};
}
