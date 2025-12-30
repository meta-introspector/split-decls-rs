// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl ClusterType { pub const STRINGS : [& 'static str ; 4] = ["development" , "devnet" , "testnet" , "mainnet-beta"] ; # [doc = " Get the known genesis hash for this ClusterType"] pub fn get_genesis_hash (& self) -> Option < Hash > { match self { Self :: MainnetBeta => { Some (Hash :: from_str ("5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d") . unwrap ()) } Self :: Testnet => { Some (Hash :: from_str ("4uhcVJyU9pJkvQyS88uRDiswHXSCkY3zQawwpjk2NsNY") . unwrap ()) } Self :: Devnet => { Some (Hash :: from_str ("EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG") . unwrap ()) } Self :: Development => None , } } }
};
}
