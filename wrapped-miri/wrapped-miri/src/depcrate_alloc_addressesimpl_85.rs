// Generated macro for impl_85 (impl)
macro_rules! Depcrate_alloc_addressesimpl_85 {
() => {
// Module: crate::alloc_addresses
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'tcx > MiriMachine < 'tcx > { pub fn free_alloc_id (& mut self , dead_id : AllocId , size : Size , align : Align , kind : MemoryKind) { let global_state = self . alloc_addresses . get_mut () ; let rng = self . rng . get_mut () ; let addr = * global_state . base_addr . get (& dead_id) . unwrap () ; let pos = global_state . int_to_ptr_map . binary_search_by_key (& addr , | (addr , _) | * addr) . unwrap () ; let removed = global_state . int_to_ptr_map . remove (pos) ; assert_eq ! (removed , (addr , dead_id)) ; global_state . exposed . remove (& dead_id) ; if let Some ((_addr_gen , reuse)) = global_state . address_generation . as_mut () { let thread = self . threads . active_thread () ; reuse . add_addr (rng , addr , size , align , kind , thread , | | { if let Some (data_race) = self . data_race . as_vclocks_ref () { data_race . release_clock (& self . threads , | clock | clock . clone ()) } else { VClock :: default () } }) } } }
};
}
