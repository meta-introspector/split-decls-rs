// Generated macro for ExpansionData (struct)
macro_rules! Depcrate_invocation_dataExpansionData {
() => {
// Module: crate::invocation_data
// Provides: {"ExpansionData"}
// Dependencies: {}
# [doc = " Contextual data associated with a macro expansion."] # [doc = ""] # [doc = " This struct provides a snapshot of the compiler's state at the time of a macro expansion,"] # [doc = " which is crucial for hygiene, error reporting, and accurate source mapping."] # [doc = ""] # [doc = " For n00bs: This is like the \"who, what, when, where\" of a macro. It keeps track of:"] # [doc = " - `id`: A unique ID for this specific expansion."] # [doc = " - `depth`: How many macros deep we are (macro calling another macro, etc.)."] # [doc = " - `module`: Information about the module where the macro is being expanded."] # [doc = " - `dir_ownership`: How the directory is handled (important for resolving paths)."] # [doc = " - `lint_node_id`: A reference to a nearby AST node for associating lints."] # [doc = " - `is_trailing_mac`: If this is a macro call at the end of a block."] # [derive (Clone , Debug)] pub struct ExpansionData { pub id : LocalExpnId , pub depth : usize , pub module : Rc < ModuleData > , pub dir_ownership : DirOwnership , # [doc = " Some parent node that is close to this macro call"] pub lint_node_id : NodeId , pub is_trailing_mac : bool , }
};
}
