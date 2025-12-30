// Generated macro for fields_to_member_skips (function)
macro_rules! Depcratefields_to_member_skips {
() => {
// Module: crate
// Provides: {"fields_to_member_skips"}
// Dependencies: {}
fn fields_to_member_skips (fields : & syn :: Fields) -> Result < Vec < bool > > { fields . iter () . map (function_skip) . collect () }
};
}
