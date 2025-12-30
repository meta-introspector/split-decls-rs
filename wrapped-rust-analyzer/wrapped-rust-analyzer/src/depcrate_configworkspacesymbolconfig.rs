// Generated macro for WorkspaceSymbolConfig (struct)
macro_rules! Depcrate_configWorkspaceSymbolConfig {
() => {
// Module: crate::config
// Provides: {"WorkspaceSymbolConfig"}
// Dependencies: {}
# [doc = " Configuration for workspace symbol search requests."] # [derive (Debug , Clone)] pub struct WorkspaceSymbolConfig { # [doc = " Should imports be excluded."] pub search_exclude_imports : bool , # [doc = " In what scope should the symbol be searched in."] pub search_scope : WorkspaceSymbolSearchScope , # [doc = " What kind of symbol is being searched for."] pub search_kind : WorkspaceSymbolSearchKind , # [doc = " How many items are returned at most."] pub search_limit : usize , }
};
}
