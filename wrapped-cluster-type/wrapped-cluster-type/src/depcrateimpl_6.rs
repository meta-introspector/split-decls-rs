// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl FromStr for ClusterType { type Err = & 'static str ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "development" => Ok (ClusterType :: Development) , "devnet" => Ok (ClusterType :: Devnet) , "testnet" => Ok (ClusterType :: Testnet) , "mainnet-beta" => Ok (ClusterType :: MainnetBeta) , _ => Err ("Unrecognized cluster type") , } } }
};
}
