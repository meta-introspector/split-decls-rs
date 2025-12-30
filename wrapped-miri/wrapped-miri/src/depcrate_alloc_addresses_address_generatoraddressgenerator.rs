// Generated macro for AddressGenerator (struct)
macro_rules! Depcrate_alloc_addresses_address_generatorAddressGenerator {
() => {
// Module: crate::alloc_addresses::address_generator
// Provides: {"AddressGenerator"}
// Dependencies: {}
# [doc = " This provides the logic to generate addresses for memory allocations in a given address range."] # [derive (Debug)] pub struct AddressGenerator { # [doc = " This is used as a memory address when a new pointer is casted to an integer. It"] # [doc = " is always larger than any address that was previously made part of a block."] next_base_addr : u64 , # [doc = " This is the last address that can be allocated."] end : u64 , }
};
}
