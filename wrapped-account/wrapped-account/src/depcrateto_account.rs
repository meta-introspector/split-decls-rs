// Generated macro for to_account (function)
macro_rules! Depcrateto_account {
() => {
// Module: crate
// Provides: {"to_account"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Serialize a `Sysvar` into an `Account`'s data."] pub fn to_account < S : SysvarSerialize , T : WritableAccount > (sysvar : & S , account : & mut T ,) -> Option < () > { bincode :: serialize_into (account . data_as_mut_slice () , sysvar) . ok () }
};
}
