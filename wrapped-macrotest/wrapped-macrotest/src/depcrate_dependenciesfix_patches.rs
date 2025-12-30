// Generated macro for fix_patches (function)
macro_rules! Depcrate_dependenciesfix_patches {
() => {
// Module: crate::dependencies
// Provides: {"fix_patches"}
// Dependencies: {}
fn fix_patches (patches : & mut Map < String , RegistryPatch > , dir : & Path) { for registry in patches . values_mut () { registry . crates . remove ("macrotest") ; for patch in registry . crates . values_mut () { patch . path = patch . path . as_ref () . map (| path | dir . join (path)) ; } } }
};
}
