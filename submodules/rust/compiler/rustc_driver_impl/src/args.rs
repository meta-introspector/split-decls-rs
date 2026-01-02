mkuse!{use std :: { env , error , fmt , fs , io } ;}
mkuse!{use rustc_session :: EarlyDiagCtxt ;}
mkitem!{mkstruct!{# [doc = " Expands argfiles in command line arguments."] # [derive (Default)] struct Expander { shell_argfiles : bool , next_is_unstable_option : bool , expanded : Vec < String > , }}}
mkitem!{mkimpl!{impl Expander { # [doc = " Handles the next argument. If the argument is an argfile, it is expanded"] # [doc = " inline."] fn arg (& mut self , arg : & str) -> Result < () , Error > { if let Some (argfile) = arg . strip_prefix ('@') { match argfile . split_once (':') { Some (("shell" , path)) if self . shell_argfiles => { shlex :: split (& Self :: read_file (path) ?) . ok_or_else (| | Error :: ShellParseError (path . to_string ())) ? . into_iter () . for_each (| arg | self . push (arg)) ; } _ => { let contents = Self :: read_file (argfile) ? ; contents . lines () . for_each (| arg | self . push (arg . to_string ())) ; } } } else { self . push (arg . to_string ()) ; } Ok (()) } # [doc = " Adds a command line argument verbatim with no argfile expansion."] fn push (& mut self , arg : String) { if self . next_is_unstable_option { self . inspect_unstable_option (& arg) ; self . next_is_unstable_option = false ; } else if let Some (unstable_option) = arg . strip_prefix ("-Z") { if unstable_option . is_empty () { self . next_is_unstable_option = true ; } else { self . inspect_unstable_option (unstable_option) ; } } self . expanded . push (arg) ; } # [doc = " Consumes the `Expander`, returning the expanded arguments."] fn finish (self) -> Vec < String > { self . expanded } # [doc = " Parses any relevant unstable flags specified on the command line."] fn inspect_unstable_option (& mut self , option : & str) { match option { "shell-argfiles" => self . shell_argfiles = true , _ => () , } } # [doc = " Reads the contents of a file as UTF-8."] fn read_file (path : & str) -> Result < String , Error > { fs :: read_to_string (path) . map_err (| e | { if e . kind () == io :: ErrorKind :: InvalidData { Error :: Utf8Error (path . to_string ()) } else { Error :: IOError (path . to_string () , e) } }) } }}}

macro_rules! arg_expand_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function arg_expand_all in module {}", module_path!());
    };
}

mkfn!{
    arg_expand_all_introspect!();
    # [doc = " Replaces any `@file` arguments with the contents of `file`, with each line of `file` as a"] # [doc = " separate argument."] # [doc = ""] # [doc = " **Note:** This function doesn't interpret argument 0 in any special way."] # [doc = " If this function is intended to be used with command line arguments,"] # [doc = " `argv[0]` must be removed prior to calling it manually."] # [allow (rustc :: untranslatable_diagnostic)] pub fn arg_expand_all (early_dcx : & EarlyDiagCtxt , at_args : & [String]) -> Vec < String > { let mut expander = Expander :: default () ; let mut result = Ok (()) ; for arg in at_args { if let Err (err) = expander . arg (arg) { result = Err (early_dcx . early_err (format ! ("failed to load argument file: {err}"))) ; } } if let Err (guar) = result { guar . raise_fatal () ; } expander . finish () }
}

macro_rules! raw_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function raw_args in module {}", module_path!());
    };
}

mkfn!{
    raw_args_introspect!();
    # [doc = " Gets the raw unprocessed command-line arguments as Unicode strings, without doing any further"] # [doc = " processing (e.g., without `@file` expansion)."] # [doc = ""] # [doc = " This function is identical to [`env::args()`] except that it emits an error when it encounters"] # [doc = " non-Unicode arguments instead of panicking."] pub fn raw_args (early_dcx : & EarlyDiagCtxt) -> Vec < String > { let mut args = Vec :: new () ; let mut guar = Ok (()) ; for (i , arg) in env :: args_os () . enumerate () { match arg . into_string () { Ok (arg) => args . push (arg) , Err (arg) => { guar = Err (early_dcx . early_err (format ! ("argument {i} is not valid Unicode: {arg:?}"))) } } } if let Err (guar) = guar { guar . raise_fatal () ; } args }
}
mkitem!{mkenum!{# [derive (Debug)] enum Error { Utf8Error (String) , IOError (String , io :: Error) , ShellParseError (String) , }}}
mkitem!{mkimpl!{impl fmt :: Display for Error { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Utf8Error (path) => write ! (fmt , "UTF-8 error in {path}") , Error :: IOError (path , err) => write ! (fmt , "IO error: {path}: {err}") , Error :: ShellParseError (path) => write ! (fmt , "invalid shell-style arguments in {path}") , } } }}}
mkitem!{mkimpl!{impl error :: Error for Error { }}}