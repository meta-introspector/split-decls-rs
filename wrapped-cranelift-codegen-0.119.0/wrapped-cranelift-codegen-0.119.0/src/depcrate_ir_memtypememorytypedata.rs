// Generated macro for MemoryTypeData (enum)
macro_rules! Depcrate_ir_memtypeMemoryTypeData {
() => {
// Module: crate::ir::memtype
// Provides: {"MemoryTypeData"}
// Dependencies: {}
# [doc = " Data defining a memory type."] # [doc = ""] # [doc = " A memory type corresponds to a layout of data in memory. It may"] # [doc = " have a statically-known or dynamically-known size."] # [derive (Clone , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum MemoryTypeData { # [doc = " An aggregate consisting of certain fields at certain offsets."] # [doc = ""] # [doc = " Fields must be sorted by offset, must be within the struct's"] # [doc = " overall size, and must not overlap. These conditions are"] # [doc = " checked by the CLIF verifier."] Struct { # [doc = " Size of this type."] size : u64 , # [doc = " Fields in this type. Sorted by offset."] fields : Vec < MemoryTypeField > , } , # [doc = " A statically-sized untyped blob of memory."] Memory { # [doc = " Accessible size."] size : u64 , } , # [doc = " A dynamically-sized untyped blob of memory, with bound given"] # [doc = " by a global value plus some static amount."] DynamicMemory { # [doc = " Static part of size."] size : u64 , # [doc = " Dynamic part of size."] gv : GlobalValue , } , # [doc = " A type with no size."] Empty , }
};
}
