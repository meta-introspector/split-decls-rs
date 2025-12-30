// Generated macro for impl_554 (impl)
macro_rules! Depcrate_repositoryimpl_554 {
() => {
// Module: crate::repository
// Provides: {"impl_554"}
// Dependencies: {}
impl std :: fmt :: Display for HexId < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { let HexId (id , long_hex) = self ; if * long_hex { id . fmt (f) } else { id . shorten_or_id () . fmt (f) } } }
};
}
