// Generated macro for Annotation (trait)
macro_rules! Depcrate_graph_sccAnnotation {
() => {
// Module: crate::graph::scc
// Provides: {"Annotation"}
// Dependencies: {}
# [doc = " An annotation for an SCC. This can be a representative,"] # [doc = " the max/min element of the SCC, or all of the above."] # [doc = ""] # [doc = " Concretely, the both merge operations must commute, e.g. where `merge`"] # [doc = " is `merge_scc` and `merge_reached`: `a.merge(b) == b.merge(a)`"] # [doc = ""] # [doc = " In general, what you want is probably always min/max according"] # [doc = " to some ordering, potentially with side constraints (min x such"] # [doc = " that P holds)."] pub trait Annotation : Debug + Copy { # [doc = " Merge two existing annotations into one during"] # [doc = " path compression.o"] fn merge_scc (self , other : Self) -> Self ; # [doc = " Merge a successor into this annotation."] fn merge_reached (self , other : Self) -> Self ; fn update_scc (& mut self , other : Self) { * self = self . merge_scc (other) } fn update_reachable (& mut self , other : Self) { * self = self . merge_reached (other) } }
};
}
