// Generated macro for align_addr (function)
macro_rules! Depcrate_alloc_addresses_address_generatoralign_addr {
() => {
// Module: crate::alloc_addresses::address_generator
// Provides: {"align_addr"}
// Dependencies: {}
# [doc = " Shifts `addr` to make it aligned with `align` by rounding `addr` to the smallest multiple"] # [doc = " of `align` that is larger or equal to `addr`"] fn align_addr (addr : u64 , align : u64) -> u64 { match addr % align { 0 => addr , rem => addr . strict_add (align) - rem , } }
};
}
