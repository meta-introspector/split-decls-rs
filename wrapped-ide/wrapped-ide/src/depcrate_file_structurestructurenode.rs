// Generated macro for StructureNode (struct)
macro_rules! Depcrate_file_structureStructureNode {
() => {
// Module: crate::file_structure
// Provides: {"StructureNode"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct StructureNode { pub parent : Option < usize > , pub label : String , pub navigation_range : TextRange , pub node_range : TextRange , pub kind : StructureNodeKind , pub detail : Option < String > , pub deprecated : bool , }
};
}
