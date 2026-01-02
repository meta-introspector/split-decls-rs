mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{pub use super :: common :: Args ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: os :: windows :: prelude :: * ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sys :: pal :: os :: current_exe ;}
mkuse!{use crate :: sys :: pal :: { ensure_no_nuls , fill_utf16_buf } ;}
mkuse!{use crate :: sys :: path :: get_long_path ;}
mkuse!{use crate :: sys :: { c , to_u16s } ;}
mkuse!{use crate :: sys_common :: AsInner ;}
mkuse!{use crate :: sys_common :: wstr :: WStrUnits ;}
mkuse!{use crate :: { io , iter , ptr } ;}

macro_rules! args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function args in module {}", module_path!());
    };
}

mkfn!{
    args_introspect!();
    pub fn args () -> Args { unsafe { let lp_cmd_line = c :: GetCommandLineW () ; let parsed_args_list = parse_lp_cmd_line (WStrUnits :: new (lp_cmd_line) , | | { current_exe () . map (PathBuf :: into_os_string) . unwrap_or_else (| _ | OsString :: new ()) }) ; Args :: new (parsed_args_list) } }
}

macro_rules! parse_lp_cmd_line_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_lp_cmd_line in module {}", module_path!());
    };
}

mkfn!{
    parse_lp_cmd_line_introspect!();
    # [doc = " Implements the Windows command-line argument parsing algorithm."] # [doc = ""] # [doc = " Microsoft's documentation for the Windows CLI argument format can be found at"] # [doc = " <https://docs.microsoft.com/en-us/cpp/cpp/main-function-command-line-args?view=msvc-160#parsing-c-command-line-arguments>"] # [doc = ""] # [doc = " A more in-depth explanation is here:"] # [doc = " <https://daviddeley.com/autohotkey/parameters/parameters.htm#WIN>"] # [doc = ""] # [doc = " Windows includes a function to do command line parsing in shell32.dll."] # [doc = " However, this is not used for two reasons:"] # [doc = ""] # [doc = " 1. Linking with that DLL causes the process to be registered as a GUI application."] # [doc = " GUI applications add a bunch of overhead, even if no windows are drawn. See"] # [doc = " <https://randomascii.wordpress.com/2018/12/03/a-not-called-function-can-cause-a-5x-slowdown/>."] # [doc = ""] # [doc = " 2. It does not follow the modern C/C++ argv rules outlined in the first two links above."] # [doc = ""] # [doc = " This function was tested for equivalence to the C/C++ parsing rules using an"] # [doc = " extensive test suite available at"] # [doc = " <https://github.com/ChrisDenton/winarg/tree/std>."] fn parse_lp_cmd_line < 'a , F : Fn () -> OsString > (lp_cmd_line : Option < WStrUnits < 'a > > , exe_name : F ,) -> Vec < OsString > { const BACKSLASH : NonZero < u16 > = NonZero :: new (b'\\' as u16) . unwrap () ; const QUOTE : NonZero < u16 > = NonZero :: new (b'"' as u16) . unwrap () ; const TAB : NonZero < u16 > = NonZero :: new (b'\t' as u16) . unwrap () ; const SPACE : NonZero < u16 > = NonZero :: new (b' ' as u16) . unwrap () ; let mut ret_val = Vec :: new () ; if lp_cmd_line . as_ref () . and_then (| cmd | cmd . peek ()) . is_none () { ret_val . push (exe_name ()) ; return ret_val ; } let mut code_units = lp_cmd_line . unwrap () ; let mut in_quotes = false ; let mut cur = Vec :: new () ; for w in & mut code_units { match w { QUOTE => in_quotes = ! in_quotes , SPACE | TAB if ! in_quotes => break , _ => cur . push (w . get ()) , } } code_units . advance_while (| w | w == SPACE || w == TAB) ; ret_val . push (OsString :: from_wide (& cur)) ; let mut cur = Vec :: new () ; let mut in_quotes = false ; while let Some (w) = code_units . next () { match w { SPACE | TAB if ! in_quotes => { ret_val . push (OsString :: from_wide (& cur [..])) ; cur . truncate (0) ; code_units . advance_while (| w | w == SPACE || w == TAB) ; } BACKSLASH => { let backslash_count = code_units . advance_while (| w | w == BACKSLASH) + 1 ; if code_units . peek () == Some (QUOTE) { cur . extend (iter :: repeat (BACKSLASH . get ()) . take (backslash_count / 2)) ; if backslash_count % 2 == 1 { code_units . next () ; cur . push (QUOTE . get ()) ; } } else { cur . extend (iter :: repeat (BACKSLASH . get ()) . take (backslash_count)) ; } } QUOTE if in_quotes => match code_units . peek () { Some (QUOTE) => { cur . push (QUOTE . get ()) ; code_units . next () ; } Some (_) => in_quotes = false , None => break , } , QUOTE => in_quotes = true , _ => cur . push (w . get ()) , } } if ! cur . is_empty () || in_quotes { ret_val . push (OsString :: from_wide (& cur [..])) ; } ret_val }
}
mkitem!{mkenum!{# [derive (Debug)] pub (crate) enum Arg { # [doc = " Add quotes (if needed)"] Regular (OsString) , # [doc = " Append raw string without quoting"] Raw (OsString) , }}}
mkitem!{mkenum!{enum Quote { Always , Auto , Never , }}}

macro_rules! append_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function append_arg in module {}", module_path!());
    };
}

mkfn!{
    append_arg_introspect!();
    pub (crate) fn append_arg (cmd : & mut Vec < u16 > , arg : & Arg , force_quotes : bool) -> io :: Result < () > { let (arg , quote) = match arg { Arg :: Regular (arg) => (arg , if force_quotes { Quote :: Always } else { Quote :: Auto }) , Arg :: Raw (arg) => (arg , Quote :: Never) , } ; ensure_no_nuls (arg) ? ; let arg_bytes = arg . as_encoded_bytes () ; let (quote , escape) = match quote { Quote :: Always => (true , true) , Quote :: Auto => { (arg_bytes . iter () . any (| c | * c == b' ' || * c == b'\t') || arg_bytes . is_empty () , true) } Quote :: Never => (false , false) , } ; if quote { cmd . push ('"' as u16) ; } let mut backslashes : usize = 0 ; for x in arg . encode_wide () { if escape { if x == '\\' as u16 { backslashes += 1 ; } else { if x == '"' as u16 { cmd . extend ((0 ..= backslashes) . map (| _ | '\\' as u16)) ; } backslashes = 0 ; } } cmd . push (x) ; } if quote { cmd . extend ((0 .. backslashes) . map (| _ | '\\' as u16)) ; cmd . push ('"' as u16) ; } Ok (()) }
}

macro_rules! append_bat_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function append_bat_arg in module {}", module_path!());
    };
}

mkfn!{
    append_bat_arg_introspect!();
    fn append_bat_arg (cmd : & mut Vec < u16 > , arg : & OsStr , mut quote : bool) -> io :: Result < () > { ensure_no_nuls (arg) ? ; if arg . is_empty () || arg . as_encoded_bytes () . last () == Some (& b'\\') { quote = true ; } for cp in arg . as_inner () . inner . code_points () { if let Some (cp) = cp . to_char () { static UNQUOTED : & str = r"#$*+-./:?@\_" ; let ascii_needs_quotes = cp . is_ascii () && ! (cp . is_ascii_alphanumeric () || UNQUOTED . contains (cp)) ; if ascii_needs_quotes || cp . is_control () { quote = true ; } } } if quote { cmd . push ('"' as u16) ; } let mut backslashes : usize = 0 ; for x in arg . encode_wide () { if x == '\\' as u16 { backslashes += 1 ; } else { if x == '"' as u16 { cmd . extend ((0 .. backslashes) . map (| _ | '\\' as u16)) ; cmd . push (b'"' as u16) } else if x == '%' as u16 || x == '\r' as u16 { cmd . extend_from_slice (& ['%' as u16 , '%' as u16 , 'c' as u16 , 'd' as u16 , ':' as u16 , '~' as u16 , ',' as u16 ,]) ; } backslashes = 0 ; } cmd . push (x) ; } if quote { cmd . extend ((0 .. backslashes) . map (| _ | '\\' as u16)) ; cmd . push ('"' as u16) ; } Ok (()) }
}

macro_rules! make_bat_command_line_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_bat_command_line in module {}", module_path!());
    };
}

mkfn!{
    make_bat_command_line_introspect!();
    pub (crate) fn make_bat_command_line (script : & [u16] , args : & [Arg] , force_quotes : bool ,) -> io :: Result < Vec < u16 > > { const INVALID_ARGUMENT_ERROR : io :: Error = io :: const_error ! (io :: ErrorKind :: InvalidInput , r#"batch file arguments are invalid"#) ; let mut cmd : Vec < u16 > = "cmd.exe /e:ON /v:OFF /d /c \"" . encode_utf16 () . collect () ; cmd . push (b'"' as u16) ; if script . contains (& (b'"' as u16)) || script . last () == Some (& (b'\\' as u16)) { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "Windows file names may not contain `\"` or end with `\\`")) ; } cmd . extend_from_slice (script . strip_suffix (& [0]) . unwrap_or (script)) ; cmd . push (b'"' as u16) ; for arg in args { cmd . push (' ' as u16) ; match arg { Arg :: Regular (arg_os) => { let arg_bytes = arg_os . as_encoded_bytes () ; const DISALLOWED : & [u8] = b"\r\n" ; if arg_bytes . iter () . any (| c | DISALLOWED . contains (c)) { return Err (INVALID_ARGUMENT_ERROR) ; } append_bat_arg (& mut cmd , arg_os , force_quotes) ? ; } _ => { append_arg (& mut cmd , arg , force_quotes) ? ; } } ; } cmd . push (b'"' as u16) ; Ok (cmd) }
}

macro_rules! to_user_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_user_path in module {}", module_path!());
    };
}

mkfn!{
    to_user_path_introspect!();
    # [doc = " Takes a path and tries to return a non-verbatim path."] # [doc = ""] # [doc = " This is necessary because cmd.exe does not support verbatim paths."] pub (crate) fn to_user_path (path : & Path) -> io :: Result < Vec < u16 > > { from_wide_to_user_path (to_u16s (path) ?) }
}

macro_rules! from_wide_to_user_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_wide_to_user_path in module {}", module_path!());
    };
}

mkfn!{
    from_wide_to_user_path_introspect!();
    pub (crate) fn from_wide_to_user_path (mut path : Vec < u16 >) -> io :: Result < Vec < u16 > > { const SEP : u16 = b'\\' as _ ; const QUERY : u16 = b'?' as _ ; const COLON : u16 = b':' as _ ; const U : u16 = b'U' as _ ; const N : u16 = b'N' as _ ; const C : u16 = b'C' as _ ; const LEGACY_MAX_PATH : usize = 260 ; if path . len () > LEGACY_MAX_PATH { return Ok (path) ; } match & path [..] { [SEP , SEP , QUERY , SEP , _ , COLON , SEP , ..] => unsafe { let lpfilename = path [4 ..] . as_ptr () ; fill_utf16_buf (| buffer , size | c :: GetFullPathNameW (lpfilename , size , buffer , ptr :: null_mut ()) , | full_path : & [u16] | { if full_path == & path [4 .. path . len () - 1] { let mut path : Vec < u16 > = full_path . into () ; path . push (0) ; path } else { path } } ,) } , [SEP , SEP , QUERY , SEP , U , N , C , SEP , ..] => unsafe { path [6] = b'\\' as u16 ; let lpfilename = path [6 ..] . as_ptr () ; fill_utf16_buf (| buffer , size | c :: GetFullPathNameW (lpfilename , size , buffer , ptr :: null_mut ()) , | full_path : & [u16] | { if full_path == & path [6 .. path . len () - 1] { let mut path : Vec < u16 > = full_path . into () ; path . push (0) ; path } else { path [6] = b'C' as u16 ; path } } ,) } , _ => get_long_path (path , false) , } }
}