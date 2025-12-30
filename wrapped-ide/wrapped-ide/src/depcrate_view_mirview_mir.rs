// Generated macro for view_mir (function)
macro_rules! Depcrate_view_mirview_mir {
() => {
// Module: crate::view_mir
// Provides: {"view_mir"}
// Dependencies: {}
pub (crate) fn view_mir (db : & RootDatabase , position : FilePosition) -> String { body_mir (db , position) . unwrap_or_else (| | "Not inside a function body" . to_owned ()) }
};
}
