// Generated macro for shared_deserialize_data (function)
macro_rules! Depcrateshared_deserialize_data {
() => {
// Module: crate
// Provides: {"shared_deserialize_data"}
// Dependencies: {}
# [cfg (feature = "bincode")] fn shared_deserialize_data < T : serde :: de :: DeserializeOwned , U : ReadableAccount > (account : & U ,) -> Result < T , bincode :: Error > { bincode :: deserialize (account . data ()) }
};
}
