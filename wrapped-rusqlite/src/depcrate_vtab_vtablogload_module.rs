// Generated macro for load_module (function)
macro_rules! Depcrate_vtab_vtablogload_module {
() => {
// Module: crate::vtab::vtablog
// Provides: {"load_module"}
// Dependencies: {}
# [doc = " Register the \"vtablog\" module."] pub fn load_module (conn : & Connection) -> Result < () > { let aux : Option < () > = None ; conn . create_module (c"vtablog" , update_module :: < VTabLog > () , aux) }
};
}
