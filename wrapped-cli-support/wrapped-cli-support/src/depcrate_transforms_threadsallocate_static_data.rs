// Generated macro for allocate_static_data (function)
macro_rules! Depcrate_transforms_threadsallocate_static_data {
() => {
// Module: crate::transforms::threads
// Provides: {"allocate_static_data"}
// Dependencies: {}
# [doc = " Allocates extra space for static data. Returns `(addr, base)`, where:"] # [doc = " - `base` is the starting address of the extra `pages`."] # [doc = " - `addr` is the _first_ address in that chunk that is aligned to `align`."] fn allocate_static_data (module : & mut Module , memory : MemoryId , pages : u32 , align : u32 ,) -> Result < (u32 , u32) , Error > { let heap_base = module . exports . iter () . filter (| e | e . name == "__heap_base") . find_map (| e | match e . item { ExportItem :: Global (id) => Some (id) , _ => None , }) ; let heap_base = match heap_base { Some (idx) => idx , None => bail ! ("failed to find `__heap_base` for injecting thread id") , } ; let (base , address) = { let global = module . globals . get_mut (heap_base) ; if global . ty != ValType :: I32 { bail ! ("the `__heap_base` global doesn't have the type `i32`") ; } if global . mutable { bail ! ("the `__heap_base` global is unexpectedly mutable") ; } let offset = match & mut global . kind { GlobalKind :: Local (ConstExpr :: Value (Value :: I32 (n))) => n , _ => bail ! ("`__heap_base` not a locally defined `i32`") , } ; let address = (* offset as u32 + (align - 1)) & ! (align - 1) ; let base = * offset ; * offset += (pages * PAGE_SIZE) as i32 ; (base , address) } ; let memory = module . memories . get_mut (memory) ; memory . initial += u64 :: from (pages) ; memory . maximum = memory . maximum . map (| m | cmp :: max (m , memory . initial)) ; Ok ((base as u32 , address)) }
};
}
