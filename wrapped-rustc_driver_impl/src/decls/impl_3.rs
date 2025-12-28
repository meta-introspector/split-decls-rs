macro_rules! deps {
    () => {
        Expander!();
        Error!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Expander { # [doc = " Handles the next argument. If the argument is an argfile, it is expanded"] # [doc = " inline."] fn arg (& mut self , arg : & str) -> Result < () , Error > { if let Some (argfile) = arg . strip_prefix ('@') { match argfile . split_once (':') { Some (("shell" , path)) if self . shell_argfiles => { shlex :: split (& Self :: read_file (path) ?) . ok_or_else (| | Error :: ShellParseError (path . to_string ())) ? . into_iter () . for_each (| arg | self . push (arg)) ; } _ => { let contents = Self :: read_file (argfile) ? ; contents . lines () . for_each (| arg | self . push (arg . to_string ())) ; } } } else { self . push (arg . to_string ()) ; } Ok (()) } # [doc = " Adds a command line argument verbatim with no argfile expansion."] fn push (& mut self , arg : String) { if self . next_is_unstable_option { self . inspect_unstable_option (& arg) ; self . next_is_unstable_option = false ; } else if let Some (unstable_option) = arg . strip_prefix ("-Z") { if unstable_option . is_empty () { self . next_is_unstable_option = true ; } else { self . inspect_unstable_option (unstable_option) ; } } self . expanded . push (arg) ; } # [doc = " Consumes the `Expander`, returning the expanded arguments."] fn finish (self) -> Vec < String > { self . expanded } # [doc = " Parses any relevant unstable flags specified on the command line."] fn inspect_unstable_option (& mut self , option : & str) { match option { "shell-argfiles" => self . shell_argfiles = true , _ => () , } } # [doc = " Reads the contents of a file as UTF-8."] fn read_file (path : & str) -> Result < String , Error > { fs :: read_to_string (path) . map_err (| e | { if e . kind () == io :: ErrorKind :: InvalidData { Error :: Utf8Error (path . to_string ()) } else { Error :: IOError (path . to_string () , e) } }) } }
    };
}

impl_3!()