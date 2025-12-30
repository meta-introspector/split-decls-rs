// Generated macro for file_structure (function)
macro_rules! Depcrate_file_structurefile_structure {
() => {
// Module: crate::file_structure
// Provides: {"file_structure"}
// Dependencies: {}
pub (crate) fn file_structure (file : & SourceFile , config : & FileStructureConfig ,) -> Vec < StructureNode > { let mut res = Vec :: new () ; let mut stack = Vec :: new () ; for event in file . syntax () . preorder_with_tokens () { match event { WalkEvent :: Enter (NodeOrToken :: Node (node)) => { if let Some (mut symbol) = structure_node (& node , config) { symbol . parent = stack . last () . copied () ; stack . push (res . len ()) ; res . push (symbol) ; } } WalkEvent :: Leave (NodeOrToken :: Node (node)) => { if structure_node (& node , config) . is_some () { stack . pop () . unwrap () ; } } WalkEvent :: Enter (NodeOrToken :: Token (token)) => { if let Some (mut symbol) = structure_token (token) { symbol . parent = stack . last () . copied () ; stack . push (res . len ()) ; res . push (symbol) ; } } WalkEvent :: Leave (NodeOrToken :: Token (token)) => { if structure_token (token) . is_some () { stack . pop () . unwrap () ; } } } } res }
};
}
