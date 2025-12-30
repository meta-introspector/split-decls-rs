// Generated macro for impl_80 (impl)
macro_rules! Depcrate_alloc_addressesimpl_80 {
() => {
// Module: crate::alloc_addresses
// Provides: {"impl_80"}
// Dependencies: {}
impl GlobalStateInner { pub fn new < 'tcx > (config : & MiriConfig , stack_addr : u64 , tcx : TyCtxt < 'tcx >) -> Self { GlobalStateInner { int_to_ptr_map : Vec :: default () , base_addr : FxHashMap :: default () , exposed : FxHashSet :: default () , provenance_mode : config . provenance_mode , address_generation : (config . native_lib . is_empty () && config . genmc_config . is_none ()) . then (| | { (AddressGenerator :: new (stack_addr .. tcx . target_usize_max ()) , ReusePool :: new (config) ,) }) , prepared_alloc_bytes : (! config . native_lib . is_empty ()) . then (FxHashMap :: default) , } } pub fn remove_unreachable_allocs (& mut self , allocs : & LiveAllocs < '_ , '_ >) { self . base_addr . retain (| id , _ | allocs . is_live (* id)) ; } }
};
}
