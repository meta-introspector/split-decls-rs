// Generated macro for impl_5 (impl)
macro_rules! Depcrate_errorimpl_5 {
() => {
// Module: crate::error
// Provides: {"impl_5"}
// Dependencies: {}
impl fmt :: Display for AddressLookupError { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> fmt :: Result { f . write_str (match self { Self :: LookupTableAccountNotFound => { "Attempted to lookup addresses from a table that does not exist" } Self :: InvalidAccountOwner => { "Attempted to lookup addresses from an account owned by the wrong program" } Self :: InvalidAccountData => "Attempted to lookup addresses from an invalid account" , Self :: InvalidLookupIndex => "Address lookup contains an invalid index" , }) } }
};
}
