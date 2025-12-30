// Generated macro for from_account (function)
macro_rules! Depcrate_statefrom_account {
() => {
// Module: crate::state
// Provides: {"from_account"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn from_account < T : ReadableAccount > (account : & T) -> Option < Feature > { if account . owner () != & id () || account . data () . len () < Feature :: size_of () { None } else { bincode :: deserialize (account . data ()) . ok () } }
};
}
