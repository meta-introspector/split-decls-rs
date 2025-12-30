// Generated macro for impl_329 (impl)
macro_rules! Depcrateimpl_329 {
() => {
// Module: crate
// Provides: {"impl_329"}
// Dependencies: {}
impl HasCrate for ModuleDef { fn krate (& self , db : & dyn HirDatabase) -> Crate { match self . module (db) { Some (module) => module . krate () , None => Crate :: core (db) . unwrap_or_else (| | db . all_crates () [0] . into ()) , } } }
};
}
