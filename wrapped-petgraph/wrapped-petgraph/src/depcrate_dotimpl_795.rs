// Generated macro for impl_795 (impl)
macro_rules! Depcrate_dotimpl_795 {
() => {
// Module: crate::dot
// Provides: {"impl_795"}
// Dependencies: {}
impl < G > fmt :: Display for Dot < '_ , G > where G : IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp , G :: EdgeWeight : fmt :: Display , G :: NodeWeight : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . graph_fmt (f , fmt :: Display :: fmt , fmt :: Display :: fmt) } }
};
}
