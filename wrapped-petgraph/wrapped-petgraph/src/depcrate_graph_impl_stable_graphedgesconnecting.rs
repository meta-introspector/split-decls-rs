// Generated macro for EdgesConnecting (struct)
macro_rules! Depcrate_graph_impl_stable_graphEdgesConnecting {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"EdgesConnecting"}
// Dependencies: {}
# [doc = " Iterator over the multiple directed edges connecting a source node to a target node"] # [derive (Debug , Clone)] pub struct EdgesConnecting < 'a , E : 'a , Ty , Ix : 'a = DefaultIx > where Ty : EdgeType , Ix : IndexType , { target_node : NodeIndex < Ix > , edges : Edges < 'a , E , Ty , Ix > , ty : PhantomData < Ty > , }
};
}
