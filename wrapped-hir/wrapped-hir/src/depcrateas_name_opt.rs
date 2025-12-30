// Generated macro for as_name_opt (function)
macro_rules! Depcrateas_name_opt {
() => {
// Module: crate
// Provides: {"as_name_opt"}
// Dependencies: {}
fn as_name_opt (name : Option < impl AsName >) -> Name { name . map_or_else (Name :: missing , | name | name . as_name ()) }
};
}
