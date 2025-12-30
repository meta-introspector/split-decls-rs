// Generated macro for as_name_opt (function)
macro_rules! Depcrate_signaturesas_name_opt {
() => {
// Module: crate::signatures
// Provides: {"as_name_opt"}
// Dependencies: {}
# [inline] fn as_name_opt (name : Option < ast :: Name >) -> Name { name . map_or_else (Name :: missing , | it | it . as_name ()) }
};
}
