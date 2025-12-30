// Generated macro for macro_7532 (macro)
macro_rules! Depcrate_missing_enforced_import_renamemacro_7532 {
() => {
// Module: crate::missing_enforced_import_rename
// Provides: {"macro_7532"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for imports that do not rename the item as specified"] # [doc = " in the `enforced-import-renames` config option."] # [doc = ""] # [doc = " Note: Even though this lint is warn-by-default, it will only trigger if"] # [doc = " import renames are defined in the `clippy.toml` file."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Consistency is important; if a project has defined import renames, then they should be"] # [doc = " followed. More practically, some item names are too vague outside of their defining scope,"] # [doc = " in which case this can enforce a more meaningful naming."] # [doc = ""] # [doc = " ### Example"] # [doc = " An example clippy.toml configuration:"] # [doc = " ```toml"] # [doc = " # clippy.toml"] # [doc = " enforced-import-renames = ["] # [doc = "     { path = \"serde_json::Value\", rename = \"JsonValue\" },"] # [doc = " ]"] # [doc = " ```"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " use serde_json::Value;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " use serde_json::Value as JsonValue;"] # [doc = " ```"] # [clippy :: version = "1.55.0"] pub MISSING_ENFORCED_IMPORT_RENAMES , style , "enforce import renames" }
};
}
