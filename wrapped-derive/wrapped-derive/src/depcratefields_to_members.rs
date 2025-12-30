// Generated macro for fields_to_members (function)
macro_rules! Depcratefields_to_members {
() => {
// Module: crate
// Provides: {"fields_to_members"}
// Dependencies: {}
fn fields_to_members (fields : & syn :: Fields) -> Vec < Member > { fields . iter () . enumerate () . map (| (i , field) | { field . ident . clone () . map_or_else (| | Member :: Unnamed (syn :: Index :: from (i)) , Member :: Named) }) . collect () }
};
}
