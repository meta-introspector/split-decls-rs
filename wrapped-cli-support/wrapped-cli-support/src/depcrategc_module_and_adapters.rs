// Generated macro for gc_module_and_adapters (function)
macro_rules! Depcrategc_module_and_adapters {
() => {
// Module: crate
// Provides: {"gc_module_and_adapters"}
// Dependencies: {}
fn gc_module_and_adapters (module : & mut Module) { loop { walrus :: passes :: gc :: run (module) ; let imports_remaining = module . imports . iter () . map (| i | i . id ()) . collect :: < HashSet < _ > > () ; let mut section = module . customs . delete_typed :: < wit :: NonstandardWitSection > () . unwrap () ; section . implements . retain (| pair | imports_remaining . contains (& pair . 0)) ; let any_removed = section . gc () ; module . customs . add (* section) ; if ! any_removed { break ; } } }
};
}
