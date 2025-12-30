// Generated macro for impl_25 (impl)
macro_rules! Depcrate_configimpl_25 {
() => {
// Module: crate::config
// Provides: {"impl_25"}
// Dependencies: {}
impl fmt :: Display for StyledFrame < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self (frame , theme) = self ; let is_dependency_code = frame . is_dependency_code () ; write ! (f , "{:>2}: " , frame . n) ? ; let name = frame . name . as_deref () . unwrap_or ("<unknown>") ; let has_hash_suffix = name . len () > 19 && & name [name . len () - 19 .. name . len () - 16] == "::h" && name [name . len () - 16 ..] . chars () . all (| x | x . is_ascii_hexdigit ()) ; let hash_suffix = if has_hash_suffix { & name [name . len () - 19 ..] } else { "<unknown>" } ; let name = if has_hash_suffix { & name [.. name . len () - 19] } else { name } ; if is_dependency_code { write ! (f , "{}" , (name) . style (theme . dependency_code)) ? ; } else { write ! (f , "{}" , (name) . style (theme . crate_code)) ? ; } write ! (f , "{}" , (hash_suffix) . style (theme . code_hash)) ? ; let mut separated = f . header ("\n") ; let file = frame . filename . as_ref () . map (| path | path . display ()) ; let file : & dyn fmt :: Display = if let Some (ref filename) = file { filename } else { & "<unknown source file>" } ; let lineno = frame . lineno . map_or ("<unknown line>" . to_owned () , | x | x . to_string ()) ; write ! (& mut separated . ready () , "    at {}:{}" , file . style (theme . file) , lineno . style (theme . line_number) ,) ? ; let v = if std :: thread :: panicking () { panic_verbosity () } else { lib_verbosity () } ; if v >= Verbosity :: Full { write ! (& mut separated . ready () , "{}" , SourceSection (frame , * theme)) ? ; } Ok (()) } }
};
}
