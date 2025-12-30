// Generated macro for impl_805 (impl)
macro_rules! Depcrate_executor_networkimpl_805 {
() => {
// Module: crate::executor::network
// Provides: {"impl_805"}
// Dependencies: {}
impl < 'a > NetworkState < 'a > { pub fn as_nic_mut (& mut self) -> Result < & mut NetworkInterface < 'a > , & 'static str > { match self { NetworkState :: Initialized (nic) => Ok (nic) , _ => Err ("Network is not initialized!") , } } }
};
}
