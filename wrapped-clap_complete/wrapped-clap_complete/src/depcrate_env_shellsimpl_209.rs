// Generated macro for impl_209 (impl)
macro_rules! Depcrate_env_shellsimpl_209 {
() => {
// Module: crate::env::shells
// Provides: {"impl_209"}
// Dependencies: {}
impl EnvCompleter for Elvish { fn name (& self) -> & 'static str { "elvish" } fn is (& self , name : & str) -> bool { name == "elvish" } fn write_registration (& self , var : & str , _name : & str , bin : & str , completer : & str , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let bin = shlex :: try_quote (bin) . unwrap_or (std :: borrow :: Cow :: Borrowed (bin)) ; let completer = shlex :: try_quote (completer) . unwrap_or (std :: borrow :: Cow :: Borrowed (completer)) ; let script = r#"
set edit:completion:arg-completer[BIN] = { |@words|
    var index = (count $words)
    set index = (- $index 1)

    put (env _CLAP_IFS="\n" _CLAP_COMPLETE_INDEX=(to-string $index) VAR="elvish" COMPLETER -- $@words) | to-lines
}
"# . replace ("COMPLETER" , & completer) . replace ("BIN" , & bin) . replace ("VAR" , var) ; writeln ! (buf , "{script}") ? ; Ok (()) } fn write_complete (& self , cmd : & mut clap :: Command , args : Vec < OsString > , current_dir : Option < & std :: path :: Path > , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let index : usize = std :: env :: var ("_CLAP_COMPLETE_INDEX") . ok () . and_then (| i | i . parse () . ok ()) . unwrap_or_default () ; let ifs : Option < String > = std :: env :: var ("_CLAP_IFS") . ok () . and_then (| i | i . parse () . ok ()) ; let completions = crate :: engine :: complete (cmd , args , index , current_dir) ? ; for (i , candidate) in completions . iter () . enumerate () { if i != 0 { write ! (buf , "{}" , ifs . as_deref () . unwrap_or ("\n")) ? ; } write ! (buf , "{}" , candidate . get_value () . to_string_lossy ()) ? ; } Ok (()) } }
};
}
