// Generated macro for add_rename_redirect (function)
macro_rules! Depcrateadd_rename_redirect {
() => {
// Module: crate
// Provides: {"add_rename_redirect"}
// Dependencies: {}
# [doc = " Adds the javascript redirection code to the given markdown output."] fn add_rename_redirect (level : Level , output : & mut String) { for (rename_level , names) in RENAMES { if * rename_level == level { let filename = level . doc_filename () . replace (".md" , ".html") ; output . push_str (RENAME_START) ; for (from , to) in * names { writeln ! (output , "        \"#{from}\": \"{filename}#{to}\",") . unwrap () ; } output . push_str (RENAME_END) ; } } }
};
}
