// Generated macro for qpath_search_pat (function)
macro_rules! Depcrate_check_proc_macroqpath_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"qpath_search_pat"}
// Dependencies: {}
# [doc = " Get the search patterns to use for the given path"] fn qpath_search_pat (path : & QPath < '_ >) -> (Pat , Pat) { match path { QPath :: Resolved (ty , path) => { let start = if ty . is_some () { Pat :: Str ("<") } else { path . segments . first () . map_or (Pat :: Str ("") , | seg | { if seg . ident . name == kw :: PathRoot { Pat :: Str ("::") } else { Pat :: Sym (seg . ident . name) } }) } ; let end = path . segments . last () . map_or (Pat :: Str ("") , | seg | { if seg . args . is_some () { Pat :: Str (">") } else { Pat :: Sym (seg . ident . name) } }) ; (start , end) } , QPath :: TypeRelative (_ , name) => (Pat :: Str ("") , Pat :: Sym (name . ident . name)) , } }
};
}
