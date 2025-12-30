// Generated macro for impl_1087 (impl)
macro_rules! Depcrate_collectionsimpl_1087 {
() => {
// Module: crate::collections
// Provides: {"impl_1087"}
// Dependencies: {}
# [stable (feature = "try_reserve" , since = "1.57.0")] # [cfg (not (test))] impl Display for TryReserveError { fn fmt (& self , fmt : & mut core :: fmt :: Formatter < '_ > ,) -> core :: result :: Result < () , core :: fmt :: Error > { fmt . write_str ("memory allocation failed") ? ; let reason = match self . kind { TryReserveErrorKind :: CapacityOverflow => { " because the computed capacity exceeded the collection's maximum" } TryReserveErrorKind :: AllocError { .. } => { " because the memory allocator returned an error" } } ; fmt . write_str (reason) } }
};
}
