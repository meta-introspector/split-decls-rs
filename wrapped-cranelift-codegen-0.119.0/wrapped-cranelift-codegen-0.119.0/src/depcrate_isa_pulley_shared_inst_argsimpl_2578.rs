// Generated macro for impl_2578 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2578 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2578"}
// Dependencies: {}
impl From < AddrG32 > for pulley_interpreter :: AddrG32 { fn from (addr : AddrG32) -> Self { match addr { AddrG32 :: RegisterBound { host_heap_base , host_heap_bound , wasm_addr , offset , } => Self { host_heap_base : host_heap_base . into () , host_heap_bound : host_heap_bound . into () , wasm_addr : wasm_addr . into () , offset , } , } } }
};
}
