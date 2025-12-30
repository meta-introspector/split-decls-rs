// Generated macro for impl_132 (impl)
macro_rules! Depcrate_raw_vecimpl_132 {
() => {
// Module: crate::raw_vec
// Provides: {"impl_132"}
// Dependencies: {}
impl fmt :: Display for TryReserveError { fn fmt (& self , fmt : & mut core :: fmt :: Formatter < '_ > ,) -> core :: result :: Result < () , core :: fmt :: Error > { fmt . write_str ("memory allocation failed") ? ; let reason = match self . kind { TryReserveErrorKind :: CapacityOverflow => { " because the computed capacity exceeded the collection's maximum" } TryReserveErrorKind :: AllocError { .. } => { " because the memory allocator returned an error" } } ; fmt . write_str (reason) } }
};
}
