// Generated macro for impl_797 (impl)
macro_rules! Depcrate_dotimpl_797 {
() => {
// Module: crate::dot
// Provides: {"impl_797"}
// Dependencies: {}
impl < G > fmt :: UpperHex for Dot < '_ , G > where G : IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp , G :: EdgeWeight : fmt :: UpperHex , G :: NodeWeight : fmt :: UpperHex , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . graph_fmt (f , fmt :: UpperHex :: fmt , fmt :: UpperHex :: fmt) } }
};
}
