// Generated macro for DAG (struct)
macro_rules! Depcrate_adt_dagDAG {
() => {
// Module: crate::adt::dag
// Provides: {"DAG"}
// Dependencies: {}
# [doc = " The Ranked-DAG data structure."] # [derive (Debug)] pub struct DAG { # [doc = " A list of nodes in the dag."] nodes : Vec < Node > , # [doc = " Places nodes in levels."] ranks : RankType , # [doc = " levels info"] levels : Vec < usize > , # [doc = " Perform validation checks."] validate : bool , }
};
}
