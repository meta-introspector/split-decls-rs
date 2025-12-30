// Generated macro for impl_974 (impl)
macro_rules! Depcrate_ir_memtypeimpl_974 {
() => {
// Module: crate::ir::memtype
// Provides: {"impl_974"}
// Dependencies: {}
impl std :: fmt :: Display for MemoryTypeData { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self { Self :: Struct { size , fields } => { write ! (f , "struct {size} {{") ? ; let mut first = true ; for field in fields { if first { first = false ; } else { write ! (f , ",") ? ; } write ! (f , " {}: {}" , field . offset , field . ty) ? ; if field . readonly { write ! (f , " readonly") ? ; } if let Some (fact) = & field . fact { write ! (f , " ! {fact}") ? ; } } write ! (f , " }}") ? ; Ok (()) } Self :: Memory { size } => { write ! (f , "memory {size:#x}") } Self :: DynamicMemory { size , gv } => { write ! (f , "dynamic_memory {gv}+{size:#x}") } Self :: Empty => { write ! (f , "empty") } } } }
};
}
