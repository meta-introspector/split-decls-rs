// Generated macro for MEMORY_ATTRIBUTE_MASK (const)
macro_rules! Depcrate_systemMEMORY_ATTRIBUTE_MASK {
() => {
// Module: crate::system
// Provides: {"MEMORY_ATTRIBUTE_MASK"}
// Dependencies: {}
# [doc = " Mask of memory attributes that specify properties of a memory region that"] # [doc = " can be managed via the CPU architecture protocol."] pub const MEMORY_ATTRIBUTE_MASK : u64 = MEMORY_ACCESS_MASK | MEMORY_SP | MEMORY_CPU_CRYPTO ;
};
}
