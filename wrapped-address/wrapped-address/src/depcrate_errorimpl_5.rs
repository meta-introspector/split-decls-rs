// Generated macro for impl_5 (impl)
macro_rules! Depcrate_errorimpl_5 {
() => {
// Module: crate::error
// Provides: {"impl_5"}
// Dependencies: {}
impl fmt :: Display for AddressError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { AddressError :: MaxSeedLengthExceeded => { f . write_str ("Length of the seed is too long for address generation") } AddressError :: InvalidSeeds => { f . write_str ("Provided seeds do not result in a valid address") } AddressError :: IllegalOwner => f . write_str ("Provided owner is not allowed") , } } }
};
}
