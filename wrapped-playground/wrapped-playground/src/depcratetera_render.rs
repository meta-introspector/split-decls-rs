// Generated macro for tera_render (function)
macro_rules! Depcratetera_render {
() => {
// Module: crate
// Provides: {"tera_render"}
// Dependencies: {}
fn tera_render (template : & str , context : & str) -> Result < String , tera :: Error > { let context = Context :: from_value (serde_json :: from_str (context) ?) ? ; Tera :: one_off (template , & context , true) }
};
}
