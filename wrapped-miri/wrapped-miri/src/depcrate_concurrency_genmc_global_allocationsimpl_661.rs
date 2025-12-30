// Generated macro for impl_661 (impl)
macro_rules! Depcrate_concurrency_genmc_global_allocationsimpl_661 {
() => {
// Module: crate::concurrency::genmc::global_allocations
// Provides: {"impl_661"}
// Dependencies: {}
impl GlobalStateInner { fn global_allocate_addr < 'tcx > (& mut self , alloc_id : AllocId , info : AllocInfo ,) -> InterpResult < 'tcx , u64 > { let entry = match self . base_addr . entry (alloc_id) { Entry :: Occupied (occupied_entry) => { return interp_ok (* occupied_entry . get ()) ; } Entry :: Vacant (vacant_entry) => vacant_entry , } ; let new_addr = self . address_generator . generate (info . size , info . align , & mut self . rng) ? ; entry . insert (new_addr) ; interp_ok (new_addr) } }
};
}
