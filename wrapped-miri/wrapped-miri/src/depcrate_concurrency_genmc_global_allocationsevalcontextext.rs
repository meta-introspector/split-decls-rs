// Generated macro for EvalContextExt (trait)
macro_rules! Depcrate_concurrency_genmc_global_allocationsEvalContextExt {
() => {
// Module: crate::concurrency::genmc::global_allocations
// Provides: {"EvalContextExt"}
// Dependencies: {}
pub (super) trait EvalContextExt < 'tcx > : crate :: MiriInterpCxExt < 'tcx > { # [doc = " Allocate a new address for the given alloc id, or return the cached address."] # [doc = " Each alloc id is assigned one unique allocation which will not change if this function is called again with the same alloc id."] fn get_global_allocation_address (& self , global_allocation_handler : & GlobalAllocationHandler , alloc_id : AllocId ,) -> InterpResult < 'tcx , u64 > { let this = self . eval_context_ref () ; let info = this . get_alloc_info (alloc_id) ; let global_state = global_allocation_handler . 0 . read () . unwrap () ; if let Some (base_addr) = global_state . base_addr . get (& alloc_id) { debug ! ("GenMC: address for global with alloc id {alloc_id:?} was cached: {base_addr} == {base_addr:#x}") ; return interp_ok (* base_addr) ; } drop (global_state) ; let mut global_state = global_allocation_handler . 0 . write () . unwrap () ; let new_addr = global_state . global_allocate_addr (alloc_id , info) ? ; debug ! ("GenMC: global with alloc id {alloc_id:?} got address: {new_addr} == {new_addr:#x}") ; assert_eq ! (GENMC_GLOBAL_ADDRESSES_MASK , new_addr & GENMC_GLOBAL_ADDRESSES_MASK , "Global address allocated outside global address space.") ; interp_ok (new_addr) } }
};
}
