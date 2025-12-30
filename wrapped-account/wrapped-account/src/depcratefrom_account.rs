// Generated macro for from_account (function)
macro_rules! Depcratefrom_account {
() => {
// Module: crate
// Provides: {"from_account"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Create a `Sysvar` from an `Account`'s data."] pub fn from_account < S : SysvarSerialize , T : ReadableAccount > (account : & T) -> Option < S > { bincode :: deserialize (account . data ()) . ok () }
};
}
