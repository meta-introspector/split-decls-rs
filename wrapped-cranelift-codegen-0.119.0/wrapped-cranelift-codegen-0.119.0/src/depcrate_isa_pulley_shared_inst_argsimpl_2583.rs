// Generated macro for impl_2583 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2583 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2583"}
// Dependencies: {}
impl From < AddrG32Bne > for pulley_interpreter :: AddrG32Bne { fn from (addr : AddrG32Bne) -> Self { match addr { AddrG32Bne :: BoundNe { host_heap_base , host_heap_bound_addr , host_heap_bound_offset , wasm_addr , offset , } => Self { host_heap_base : host_heap_base . into () , host_heap_bound_addr : host_heap_bound_addr . into () , host_heap_bound_offset , wasm_addr : wasm_addr . into () , offset , } , } } }
};
}
