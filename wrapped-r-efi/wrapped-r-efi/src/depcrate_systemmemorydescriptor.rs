// Generated macro for MemoryDescriptor (struct)
macro_rules! Depcrate_systemMemoryDescriptor {
() => {
// Module: crate::system
// Provides: {"MemoryDescriptor"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct MemoryDescriptor { pub r#type : u32 , pub physical_start : crate :: base :: PhysicalAddress , pub virtual_start : crate :: base :: VirtualAddress , pub number_of_pages : u64 , pub attribute : u64 , }
};
}
