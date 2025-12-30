// Generated macro for call_hierarchy (function)
macro_rules! Depcrate_call_hierarchycall_hierarchy {
() => {
// Module: crate::call_hierarchy
// Provides: {"call_hierarchy"}
// Dependencies: {}
pub (crate) fn call_hierarchy (db : & RootDatabase , position : FilePosition , config : & CallHierarchyConfig < '_ > ,) -> Option < RangeInfo < Vec < NavigationTarget > > > { goto_definition :: goto_definition (db , position , & GotoDefinitionConfig { minicore : config . minicore } ,) }
};
}
