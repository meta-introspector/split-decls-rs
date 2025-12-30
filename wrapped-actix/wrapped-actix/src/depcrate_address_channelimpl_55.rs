// Generated macro for impl_55 (impl)
macro_rules! Depcrate_address_channelimpl_55 {
() => {
// Module: crate::address::channel
// Provides: {"impl_55"}
// Dependencies: {}
impl < A : Actor > Hash for AddressSender < A > { fn hash < H : Hasher > (& self , state : & mut H) { let hash : * const Inner < A > = self . inner . as_ref () ; hash . hash (state) ; } }
};
}
