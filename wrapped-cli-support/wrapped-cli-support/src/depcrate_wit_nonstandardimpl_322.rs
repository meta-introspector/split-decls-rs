// Generated macro for impl_322 (impl)
macro_rules! Depcrate_wit_nonstandardimpl_322 {
() => {
// Module: crate::wit::nonstandard
// Provides: {"impl_322"}
// Dependencies: {}
impl walrus :: CustomSection for WasmBindgenAux { fn name (& self) -> & str { "wasm-bindgen custom section" } fn data (& self , _ : & walrus :: IdsToIndices) -> Cow < '_ , [u8] > { panic ! ("shouldn't emit custom sections just yet") ; } fn add_gc_roots (& self , roots : & mut walrus :: passes :: Roots) { if let Some (id) = self . externref_table { roots . push_table (id) ; } if let Some (id) = self . function_table { roots . push_table (id) ; } if let Some (id) = self . externref_alloc { roots . push_func (id) ; } if let Some (id) = self . externref_drop_slice { roots . push_func (id) ; } if let Some (id) = self . exn_store { roots . push_func (id) ; } if let Some (id) = self . stack_pointer { roots . push_global (id) ; } if let Some (id) = self . thread_destroy { roots . push_func (id) ; } } }
};
}
