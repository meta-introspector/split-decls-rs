// Generated macro for convert_bad_chars (function)
macro_rules! Depcrateconvert_bad_chars {
() => {
// Module: crate
// Provides: {"convert_bad_chars"}
// Dependencies: {}
fn convert_bad_chars (name : & str) -> String { let name = name . replace ('/' , "__") . replace ('+' , "Plus") ; if let Some (pos) = name . find ('-') { if name [pos + 1 ..] . chars () . next () . map (char :: is_numeric) . unwrap_or (false) { name . replace ('-' , "Minus") } else { name . replace ('-' , "") } } else { name } }
};
}
