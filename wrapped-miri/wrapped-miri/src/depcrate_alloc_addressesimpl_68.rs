// Generated macro for impl_68 (impl)
macro_rules! Depcrate_alloc_addressesimpl_68 {
() => {
// Module: crate::alloc_addresses
// Provides: {"impl_68"}
// Dependencies: {}
impl GlobalStateInner { pub fn new (config : & MiriConfig , stack_addr : u64) -> Self { GlobalStateInner { int_to_ptr_map : Vec :: default () , base_addr : FxHashMap :: default () , prepared_alloc_bytes : FxHashMap :: default () , reuse : ReusePool :: new (config) , exposed : FxHashSet :: default () , next_base_addr : stack_addr , provenance_mode : config . provenance_mode , } } pub fn remove_unreachable_allocs (& mut self , allocs : & LiveAllocs < '_ , '_ >) { self . base_addr . retain (| id , _ | allocs . is_live (* id)) ; } }
};
}
