// Generated macro for get_directives (function)
macro_rules! Depcrateget_directives {
() => {
// Module: crate
// Provides: {"get_directives"}
// Dependencies: {}
# [doc = " Get a list of directives from a file."] fn get_directives (template : & str) -> Result < Vec < Directive > , () > { let mut directives = Vec :: new () ; let mut errors = false ; let file = fs :: read_to_string (template) . unwrap () ; for (lineno , line) in file . split ('\n') . enumerate () { let lineno = lineno + 1 ; if DEPRECATED_LINE_PATTERN . is_match (line) { print_err ("Deprecated directive syntax, replace `// @` with `//@ `" , lineno) ; errors = true ; continue ; } let Some (cap) = LINE_PATTERN . captures (line) else { continue ; } ; let negated = & cap ["negated"] == "!" ; let args_str = cap . name ("args") . map (| m | m . as_str ()) . unwrap_or_default () ; let Some (args) = shlex :: split (args_str) else { print_err (& format ! ("Invalid arguments to shlex::split: `{args_str}`" ,) , lineno) ; errors = true ; continue ; } ; if let Some ((kind , path)) = DirectiveKind :: parse (& cap ["directive"] , negated , & args) { directives . push (Directive { kind , lineno , path : path . to_owned () }) } } if ! errors { Ok (directives) } else { Err (()) } }
};
}
