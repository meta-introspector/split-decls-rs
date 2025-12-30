// Generated macro for impl_98 (impl)
macro_rules! Depcrate_nodes_hamtimpl_98 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_98"}
// Dependencies: {}
impl < A : HashValue + fmt :: Debug > fmt :: Debug for Node < A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { write ! (f , "Node[ ") ? ; for i in self . data . indices () { write ! (f , "{}: " , i) ? ; match & self . data [i] { Entry :: Value (v , h) => write ! (f , "{:?} :: {}, " , v , h) ? , Entry :: Collision (c) => write ! (f , "Coll{:?} :: {}" , c . data , c . hash) ? , Entry :: Node (n) => write ! (f , "{:?}, " , n) ? , } } write ! (f , " ]") } }
};
}
