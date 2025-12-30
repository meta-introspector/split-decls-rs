// Generated macro for impl_56 (impl)
macro_rules! Depcrate_alloc_addresses_address_generatorimpl_56 {
() => {
// Module: crate::alloc_addresses::address_generator
// Provides: {"impl_56"}
// Dependencies: {}
impl AddressGenerator { pub fn new (addr_range : Range < u64 >) -> Self { Self { next_base_addr : addr_range . start , end : addr_range . end } } # [doc = " Get the remaining range where this `AddressGenerator` can still allocate addresses."] pub fn get_remaining (& self) -> Range < u64 > { self . next_base_addr .. self . end } # [doc = " Generate a new address with the specified size and alignment, using the given Rng to add some randomness."] # [doc = " The returned allocation is guaranteed not to overlap with any address ranges given out by the generator before."] # [doc = " Returns an error if the allocation request cannot be fulfilled."] pub fn generate < 'tcx , R : Rng > (& mut self , size : Size , align : Align , rng : & mut R ,) -> InterpResult < 'tcx , u64 > { let slack = rng . random_range (0 .. 16) ; let base_addr = self . next_base_addr . checked_add (slack) . ok_or_else (| | err_exhaust ! (AddressSpaceFull)) ? ; let base_addr = align_addr (base_addr , align . bytes ()) ; self . next_base_addr = base_addr . checked_add (size . bytes () . max (1)) . ok_or_else (| | err_exhaust ! (AddressSpaceFull)) ? ; if self . next_base_addr > self . end { throw_exhaust ! (AddressSpaceFull) ; } interp_ok (base_addr) } }
};
}
