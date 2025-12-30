// Generated macro for load_module (function)
macro_rules! Depcrate_vtab_seriesload_module {
() => {
// Module: crate::vtab::series
// Provides: {"load_module"}
// Dependencies: {}
# [doc = " Register the `generate_series` module."] pub fn load_module (conn : & Connection) -> Result < () > { let aux : Option < () > = None ; conn . create_module (c"generate_series" , eponymous_only_module :: < SeriesTab > () , aux ,) }
};
}
