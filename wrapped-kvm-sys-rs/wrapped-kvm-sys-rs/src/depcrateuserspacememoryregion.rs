// Generated macro for UserspaceMemoryRegion (struct)
macro_rules! DepcrateUserspaceMemoryRegion {
() => {
// Module: crate
// Provides: {"UserspaceMemoryRegion"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug , Default)] struct UserspaceMemoryRegion { pub slot : u32 , pub flags : u32 , pub guest_phys_addr : u64 , pub memory_size : u64 , pub userspace_addr : u64 , }
};
}
