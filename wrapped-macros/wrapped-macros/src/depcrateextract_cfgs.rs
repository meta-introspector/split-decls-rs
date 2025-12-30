// Generated macro for extract_cfgs (function)
macro_rules! Depcrateextract_cfgs {
() => {
// Module: crate
// Provides: {"extract_cfgs"}
// Dependencies: {}
fn extract_cfgs (attrs : & [Attribute]) -> Vec < Attribute > { let mut cfgs = vec ! [] ; for attr in attrs { if attr . path () . is_ident ("cfg") { cfgs . push (attr . clone ()) ; } } cfgs }
};
}
