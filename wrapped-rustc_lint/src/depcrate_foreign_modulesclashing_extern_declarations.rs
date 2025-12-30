// Generated macro for clashing_extern_declarations (function)
macro_rules! Depcrate_foreign_modulesclashing_extern_declarations {
() => {
// Module: crate::foreign_modules
// Provides: {"clashing_extern_declarations"}
// Dependencies: {}
fn clashing_extern_declarations (tcx : TyCtxt < '_ > , () : ()) { let mut lint = ClashingExternDeclarations :: new () ; for id in tcx . hir_crate_items (()) . foreign_items () { lint . check_foreign_item (tcx , id) ; } }
};
}
