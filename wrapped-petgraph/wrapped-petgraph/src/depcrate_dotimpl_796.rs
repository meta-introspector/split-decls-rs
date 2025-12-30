// Generated macro for impl_796 (impl)
macro_rules! Depcrate_dotimpl_796 {
() => {
// Module: crate::dot
// Provides: {"impl_796"}
// Dependencies: {}
impl < G > fmt :: LowerHex for Dot < '_ , G > where G : IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp , G :: EdgeWeight : fmt :: LowerHex , G :: NodeWeight : fmt :: LowerHex , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . graph_fmt (f , fmt :: LowerHex :: fmt , fmt :: LowerHex :: fmt) } }
};
}
