// Generated macro for impl_2584 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2584 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2584"}
// Dependencies: {}
impl fmt :: Display for AddrG32Bne { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AddrG32Bne :: BoundNe { host_heap_base , host_heap_bound_addr , host_heap_bound_offset , wasm_addr , offset , } => { let host_heap_base = reg_name (* * host_heap_base) ; let host_heap_bound_addr = reg_name (* * host_heap_bound_addr) ; let wasm_addr = reg_name (* * wasm_addr) ; write ! (f , "{host_heap_base}, \
                     *[{host_heap_bound_addr} + {host_heap_bound_offset}], \
                     {wasm_addr}, \
                     {offset}" ,) } } } }
};
}
