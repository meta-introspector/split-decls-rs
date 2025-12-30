// Generated macro for macro_126 (macro)
macro_rules! Depcrate_visitmacro_126 {
() => {
// Module: crate::visit
// Provides: {"macro_126"}
// Dependencies: {}
trait_template ! { # [doc = " A graph that can create a map that tracks the visited status of its nodes."] # [allow (clippy :: needless_arbitrary_self_type)] pub trait Visitable : GraphBase { @ section type # [doc = " The associated map type"] type Map : VisitMap < Self :: NodeId >; @ section self # [doc = " Create a new visitor map"] fn visit_map (self : & Self) -> Self :: Map ; # [doc = " Reset the visitor map (and resize to new size of graph if needed)"] fn reset_map (self : & Self , map : & mut Self :: Map) ; } }
};
}
