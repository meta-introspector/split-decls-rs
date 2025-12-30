// Generated macro for split_lines (function)
macro_rules! Depcratesplit_lines {
() => {
// Module: crate
// Provides: {"split_lines"}
// Dependencies: {}
fn split_lines (styled : & [adapter :: Element]) -> Vec < Vec < adapter :: Element > > { let mut lines = Vec :: new () ; let mut current_line = Vec :: new () ; for mut element in styled . iter () . cloned () { while let Some ((current , remaining)) = element . text . split_once ('\n') { let current = current . strip_suffix ('\r') . unwrap_or (current) ; let mut new_element = element . clone () ; new_element . text = current . to_owned () ; current_line . push (new_element) ; lines . push (current_line) ; current_line = Vec :: new () ; element . text = remaining . to_owned () ; } current_line . push (element) ; } if ! current_line . is_empty () { lines . push (current_line) ; } lines }
};
}
