// Generated macro for is_explicit_trivial (function)
macro_rules! Depcrateis_explicit_trivial {
() => {
// Module: crate
// Provides: {"is_explicit_trivial"}
// Dependencies: {}
fn is_explicit_trivial (field : & Field) -> Result < bool > { for attr in & field . attrs { if attr . path () . is_ident ("trivial") { attr . meta . require_path_only () ? ; return Ok (true) ; } } Ok (false) }
};
}
