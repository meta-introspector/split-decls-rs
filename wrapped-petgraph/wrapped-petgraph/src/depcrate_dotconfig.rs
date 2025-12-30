// Generated macro for Config (enum)
macro_rules! Depcrate_dotConfig {
() => {
// Module: crate::dot
// Provides: {"Config"}
// Dependencies: {}
# [doc = " `Dot` configuration."] # [doc = ""] # [doc = " This enum does not have an exhaustive definition (will be expanded)"] # [non_exhaustive] # [derive (Debug , PartialEq , Eq)] pub enum Config { # [doc = " Use indices for node labels."] NodeIndexLabel , # [doc = " Use indices for edge labels."] EdgeIndexLabel , # [doc = " Do not generate `label` attributes for edges."] EdgeNoLabel , # [doc = " Do not generate `label` attributes for nodes."] NodeNoLabel , # [doc = " Do not print the graph/digraph string."] GraphContentOnly , # [doc = " Sets direction of graph layout."] RankDir (RankDir) , }
};
}
