// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl Source { fn src (& mut self , stype : SourceType) -> & mut wit_bindgen_core :: Source { match stype { SourceType :: HDefs => & mut self . h_defs , SourceType :: HFns => & mut self . h_fns , } } fn append (& mut self , append_src : & Source) { self . h_defs . push_str (& append_src . h_defs) ; self . h_fns . push_str (& append_src . h_fns) ; self . h_helpers . push_str (& append_src . h_helpers) ; self . h_async . push_str (& append_src . h_async) ; self . c_defs . push_str (& append_src . c_defs) ; self . c_fns . push_str (& append_src . c_fns) ; self . c_helpers . push_str (& append_src . c_helpers) ; self . c_adapters . push_str (& append_src . c_adapters) ; self . c_async . push_str (& append_src . c_async) ; } fn h_defs (& mut self , s : & str) { self . h_defs . push_str (s) ; } fn h_fns (& mut self , s : & str) { self . h_fns . push_str (s) ; } fn h_helpers (& mut self , s : & str) { self . h_helpers . push_str (s) ; } fn c_fns (& mut self , s : & str) { self . c_fns . push_str (s) ; } fn c_helpers (& mut self , s : & str) { self . c_helpers . push_str (s) ; } fn c_adapters (& mut self , s : & str) { self . c_adapters . push_str (s) ; } }
};
}
