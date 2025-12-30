// Generated macro for impl_798 (impl)
macro_rules! Depcrate_dotimpl_798 {
() => {
// Module: crate::dot
// Provides: {"impl_798"}
// Dependencies: {}
impl < G > fmt :: Debug for Dot < '_ , G > where G : IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp , G :: EdgeWeight : fmt :: Debug , G :: NodeWeight : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . graph_fmt (f , fmt :: Debug :: fmt , fmt :: Debug :: fmt) } }
};
}
