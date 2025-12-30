// Generated macro for impl_118 (impl)
macro_rules! Depcrate_constraintsimpl_118 {
() => {
// Module: crate::constraints
// Provides: {"impl_118"}
// Dependencies: {}
impl < 'tcx > OutlivesConstraintSet < 'tcx > { pub (crate) fn push (& mut self , constraint : OutlivesConstraint < 'tcx >) { debug ! ("OutlivesConstraintSet::push({:?})" , constraint) ; if constraint . sup == constraint . sub { return ; } self . outlives . push (constraint) ; } # [doc = " Constructs a \"normal\" graph from the constraint set; the graph makes it"] # [doc = " easy to find the constraints affecting a particular region."] # [doc = ""] # [doc = " N.B., this graph contains a \"frozen\" view of the current"] # [doc = " constraints. Any new constraints added to the `OutlivesConstraintSet`"] # [doc = " after the graph is built will not be present in the graph."] pub (crate) fn graph (& self , num_region_vars : usize) -> graph :: NormalConstraintGraph { graph :: ConstraintGraph :: new (graph :: Normal , self , num_region_vars) } # [doc = " Like `graph`, but constraints a reverse graph where `R1: R2`"] # [doc = " represents an edge `R2 -> R1`."] pub (crate) fn reverse_graph (& self , num_region_vars : usize) -> graph :: ReverseConstraintGraph { graph :: ConstraintGraph :: new (graph :: Reverse , self , num_region_vars) } pub (crate) fn outlives (& self ,) -> & IndexSlice < OutlivesConstraintIndex , OutlivesConstraint < 'tcx > > { & self . outlives } }
};
}
