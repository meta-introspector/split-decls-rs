// Generated macro for extract_section (function)
macro_rules! Depcrateextract_section {
() => {
// Module: crate
// Provides: {"extract_section"}
// Dependencies: {}
pub fn extract_section (file : & Path) -> Result < Section , Error > { let f = fs :: File :: open (file) . with_context (| | format ! ("could not open `{}`" , file . display ())) ? ; let mut f = io :: BufReader :: new (f) ; let mut line = String :: new () ; f . read_line (& mut line) ? ; if ! line . starts_with ("# ") { bail ! ("expected input file to start with # header") ; } let (_name , section) = util :: parse_name_and_section (& line [2 ..] . trim ()) . with_context (| | { format ! ("expected input file to have header with the format `# command-name(1)`, found: `{}`" , line) }) ? ; Ok (section) }
};
}
