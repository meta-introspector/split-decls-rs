// Generated macro for impl_213 (impl)
macro_rules! Depcrate_connect_infoimpl_213 {
() => {
// Module: crate::connect::info
// Provides: {"impl_213"}
// Dependencies: {}
impl < R : Host > fmt :: Display for ConnectInfo < R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}:{}" , self . hostname () , self . port ()) } }
};
}
