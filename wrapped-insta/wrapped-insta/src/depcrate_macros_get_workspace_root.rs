// Generated macro for _get_workspace_root (macro)
macro_rules! Depcrate_macros_get_workspace_root {
() => {
// Module: crate::macros
// Provides: {"_get_workspace_root"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! _get_workspace_root { () => { { use $ crate :: _macro_support :: { env , option_env } ; const WORKSPACE_ROOT : $ crate :: _macro_support :: Workspace = if let Some (root) = option_env ! ("INSTA_WORKSPACE_ROOT") { $ crate :: _macro_support :: Workspace :: UseAsIs (root) } else { $ crate :: _macro_support :: Workspace :: DetectWithCargo (env ! ("CARGO_MANIFEST_DIR")) } ; $ crate :: _macro_support :: get_cargo_workspace (WORKSPACE_ROOT) } } ; }
};
}
