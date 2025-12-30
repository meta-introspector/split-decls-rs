// Generated macro for load_module (function)
macro_rules! Depcrate_vtab_arrayload_module {
() => {
// Module: crate::vtab::array
// Provides: {"load_module"}
// Dependencies: {}
# [doc = " Register the \"rarray\" module."] pub fn load_module (conn : & Connection) -> Result < () > { let aux : Option < () > = None ; conn . create_module (c"rarray" , eponymous_only_module :: < ArrayTab > () , aux) }
};
}
