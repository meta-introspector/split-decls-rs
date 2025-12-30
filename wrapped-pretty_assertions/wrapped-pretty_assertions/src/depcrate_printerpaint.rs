// Generated macro for paint (macro)
macro_rules! Depcrate_printerpaint {
() => {
// Module: crate::printer
// Provides: {"paint"}
// Dependencies: {}
macro_rules ! paint { ($ f : expr , $ style : expr , $ fmt : expr , $ ($ args : tt) *) => (write ! ($ f , "{}" , format ! ($ fmt , $ ($ args) *) . paint ($ style))) }
};
}
