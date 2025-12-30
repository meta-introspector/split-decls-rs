// Generated macro for fix_replacements (function)
macro_rules! Depcrate_dependenciesfix_replacements {
() => {
// Module: crate::dependencies
// Provides: {"fix_replacements"}
// Dependencies: {}
fn fix_replacements (replacements : & mut Map < String , Patch > , dir : & Path) { replacements . remove ("macrotest") ; for replacement in replacements . values_mut () { replacement . path = replacement . path . as_ref () . map (| path | dir . join (path)) ; } }
};
}
