// Generated macro for impl_34 (impl)
macro_rules! Depcrate_alloc_alloc_bytesimpl_34 {
() => {
// Module: crate::alloc::alloc_bytes
// Provides: {"impl_34"}
// Dependencies: {}
impl MiriAllocBytes { # [doc = " This method factors out how a `MiriAllocBytes` object is allocated, given a specific allocation function."] # [doc = " If `size == 0` we allocate using a different `alloc_layout` with `size = 1`, to ensure each allocation has a unique address."] # [doc = " Returns `Err(alloc_layout)` if the allocation function returns a `ptr` where `ptr.is_null()`."] fn alloc_with (size : u64 , align : u64 , params : MiriAllocParams , alloc_fn : impl FnOnce (Layout , & MiriAllocParams) -> * mut u8 ,) -> Result < MiriAllocBytes , () > { let size = usize :: try_from (size) . map_err (| _ | ()) ? ; let align = usize :: try_from (align) . map_err (| _ | ()) ? ; let layout = Layout :: from_size_align (size , align) . map_err (| _ | ()) ? ; let alloc_layout = if size == 0 { Layout :: from_size_align (1 , align) . unwrap () } else { layout } ; let ptr = alloc_fn (alloc_layout , & params) ; if ptr . is_null () { Err (()) } else { Ok (Self { ptr , layout , params }) } } }
};
}
