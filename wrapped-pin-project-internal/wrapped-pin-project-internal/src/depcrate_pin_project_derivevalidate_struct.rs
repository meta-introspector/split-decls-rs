// Generated macro for validate_struct (function)
macro_rules! Depcrate_pin_project_derivevalidate_struct {
() => {
// Module: crate::pin_project::derive
// Provides: {"validate_struct"}
// Dependencies: {}
fn validate_struct (ident : & Ident , fields : & Fields) -> Result < () > { if fields . is_empty () { let msg = "#[pin_project] attribute may not be used on structs with zero fields" ; if let Fields :: Unit = fields { bail ! (ident , msg) } bail ! (fields , msg) } Ok (()) }
};
}
