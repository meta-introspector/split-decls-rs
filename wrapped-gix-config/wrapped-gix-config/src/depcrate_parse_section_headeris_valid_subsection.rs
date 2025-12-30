// Generated macro for is_valid_subsection (function)
macro_rules! Depcrate_parse_section_headeris_valid_subsection {
() => {
// Module: crate::parse::section::header
// Provides: {"is_valid_subsection"}
// Dependencies: {}
# [doc = " Return true if `name` is valid as subsection name, like `origin` in `[remote \"origin\"]`."] pub fn is_valid_subsection (name : & BStr) -> bool { name . find_byteset (b"\n\0") . is_none () }
};
}
