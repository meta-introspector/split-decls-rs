// Generated macro for impl_41 (impl)
macro_rules! Depcrate_queueimpl_41 {
() => {
// Module: crate::queue
// Provides: {"impl_41"}
// Dependencies: {}
impl < K : Ord + std :: fmt :: Debug , T : std :: fmt :: Debug > std :: fmt :: Debug for Item < K , T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "({:?}: {:?})" , self . key , self . value) } }
};
}
