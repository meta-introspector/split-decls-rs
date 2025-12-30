// Generated macro for MemFlags (struct)
macro_rules! Depcrate_ir_memflagsMemFlags {
() => {
// Module: crate::ir::memflags
// Provides: {"MemFlags"}
// Dependencies: {}
# [doc = " Flags for memory operations like load/store."] # [doc = ""] # [doc = " Each of these flags introduce a limited form of undefined behavior. The flags each enable"] # [doc = " certain optimizations that need to make additional assumptions. Generally, the semantics of a"] # [doc = " program does not change when a flag is removed, but adding a flag will."] # [doc = ""] # [doc = " In addition, the flags determine the endianness of the memory access.  By default,"] # [doc = " any memory access uses the native endianness determined by the target ISA.  This can"] # [doc = " be overridden for individual accesses by explicitly specifying little- or big-endian"] # [doc = " semantics via the flags."] # [derive (Clone , Copy , Debug , Hash , PartialEq , Eq)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct MemFlags { bits : u16 , }
};
}
