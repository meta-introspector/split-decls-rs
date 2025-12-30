// Generated macro for impl_2579 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2579 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2579"}
// Dependencies: {}
impl fmt :: Display for AddrG32 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AddrG32 :: RegisterBound { host_heap_base , host_heap_bound , wasm_addr , offset , } => { let host_heap_base = reg_name (* * host_heap_base) ; let host_heap_bound = reg_name (* * host_heap_bound) ; let wasm_addr = reg_name (* * wasm_addr) ; write ! (f , "{host_heap_base}, {host_heap_bound}, {wasm_addr}, {offset}" ,) } } } }
};
}
