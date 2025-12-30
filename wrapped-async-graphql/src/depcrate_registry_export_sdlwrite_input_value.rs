// Generated macro for write_input_value (function)
macro_rules! Depcrate_registry_export_sdlwrite_input_value {
() => {
// Module: crate::registry::export_sdl
// Provides: {"write_input_value"}
// Dependencies: {}
fn write_input_value (sdl : & mut String , input_value : & MetaInputValue) { if let Some (default_value) = & input_value . default_value { _ = write ! (sdl , "{}: {} = {}" , input_value . name , input_value . ty , default_value) ; } else { _ = write ! (sdl , "{}: {}" , input_value . name , input_value . ty) ; } write_deprecated (sdl , & input_value . deprecation) ; }
};
}
