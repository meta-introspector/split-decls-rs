// Generated macro for if_present_on_unit_struct (function)
macro_rules! Depcrate_errorif_present_on_unit_struct {
() => {
// Module: crate::error
// Provides: {"if_present_on_unit_struct"}
// Dependencies: {}
# [doc = " Ensures that parameters or filter is not present on a unit struct."] pub fn if_present_on_unit_struct (ctx : Ctx , attrs : & ParsedAttributes) { if attrs . params . is_set () { params_on_unit_struct (ctx) } if ! attrs . filter . is_empty () { filter_on_unit_struct (ctx) } }
};
}
