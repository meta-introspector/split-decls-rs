// Generated macro for get_all_declarations (function)
macro_rules! Depcrateget_all_declarations {
() => {
// Module: crate
// Provides: {"get_all_declarations"}
// Dependencies: {}
pub fn get_all_declarations () -> Vec < DeclInfo > { DECL_REGISTRY . lock () . map (| r | r . declarations . clone ()) . unwrap_or_default () }
};
}
