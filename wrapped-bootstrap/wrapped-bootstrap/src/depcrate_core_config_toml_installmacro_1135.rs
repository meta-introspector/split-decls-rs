// Generated macro for macro_1135 (macro)
macro_rules! Depcrate_core_config_toml_installmacro_1135 {
() => {
// Module: crate::core::config::toml::install
// Provides: {"macro_1135"}
// Dependencies: {}
define_config ! { # [doc = " TOML representation of various global install decisions."] # [derive (Default)] struct Install { prefix : Option < String > = "prefix" , sysconfdir : Option < String > = "sysconfdir" , docdir : Option < String > = "docdir" , bindir : Option < String > = "bindir" , libdir : Option < String > = "libdir" , mandir : Option < String > = "mandir" , datadir : Option < String > = "datadir" , } }
};
}
