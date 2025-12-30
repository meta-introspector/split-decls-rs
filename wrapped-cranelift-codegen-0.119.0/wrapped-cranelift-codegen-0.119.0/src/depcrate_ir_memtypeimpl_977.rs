// Generated macro for impl_977 (impl)
macro_rules! Depcrate_ir_memtypeimpl_977 {
() => {
// Module: crate::ir::memtype
// Provides: {"impl_977"}
// Dependencies: {}
impl MemoryTypeData { # [doc = " Provide the static size of this type, if known."] # [doc = ""] # [doc = " (The size may not be known for dynamically-sized arrays or"] # [doc = " memories, when those memtype kinds are added.)"] pub fn static_size (& self) -> Option < u64 > { match self { Self :: Struct { size , .. } => Some (* size) , Self :: Memory { size } => Some (* size) , Self :: DynamicMemory { .. } => None , Self :: Empty => Some (0) , } } }
};
}
