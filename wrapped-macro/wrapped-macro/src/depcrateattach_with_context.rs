// Generated macro for attach_with_context (function)
macro_rules! Depcrateattach_with_context {
() => {
// Module: crate
// Provides: {"attach_with_context"}
// Dependencies: {}
fn attach_with_context (err : anyhow :: Error) -> anyhow :: Error { if let Some (e) = err . downcast_ref :: < wit_bindgen_rust :: MissingWith > () { let option = e . 0 . clone () ; return err . context (format ! ("missing one of:\n\
            * `generate_all` option\n\
            * `with: {{ \"{option}\": path::to::bindings, }}`\n\
            * `with: {{ \"{option}\": generate, }}`\
            ")) ; } err }
};
}
