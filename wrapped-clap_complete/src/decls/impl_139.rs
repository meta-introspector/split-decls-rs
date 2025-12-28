macro_rules! deps {
    () => {
        Zsh!();
        EnvCompleter!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl EnvCompleter for Zsh { fn name (& self) -> & 'static str { "zsh" } fn is (& self , name : & str) -> bool { name == "zsh" } fn write_registration (& self , var : & str , name : & str , bin : & str , completer : & str , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let escaped_name = name . replace ('-' , "_") ; let bin = shlex :: try_quote (bin) . unwrap_or (std :: borrow :: Cow :: Borrowed (bin)) ; let completer = shlex :: try_quote (completer) . unwrap_or (std :: borrow :: Cow :: Borrowed (completer)) ; let script = r#"#compdef BIN
function _clap_dynamic_completer_NAME() {
    local _CLAP_COMPLETE_INDEX=$(expr $CURRENT - 1)
    local _CLAP_IFS=$'\n'

    local completions=("${(@f)$( \
        _CLAP_IFS="$_CLAP_IFS" \
        _CLAP_COMPLETE_INDEX="$_CLAP_COMPLETE_INDEX" \
        VAR="zsh" \
        COMPLETER -- "${words[@]}" 2>/dev/null \
    )}")

    if [[ -n $completions ]]; then
        _describe 'values' completions
    fi
}

compdef _clap_dynamic_completer_NAME BIN"# . replace ("NAME" , & escaped_name) . replace ("COMPLETER" , & completer) . replace ("BIN" , & bin) . replace ("VAR" , var) ; writeln ! (buf , "{script}") ? ; Ok (()) } fn write_complete (& self , cmd : & mut clap :: Command , args : Vec < OsString > , current_dir : Option < & std :: path :: Path > , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let index : usize = std :: env :: var ("_CLAP_COMPLETE_INDEX") . ok () . and_then (| i | i . parse () . ok ()) . unwrap_or_default () ; let ifs : Option < String > = std :: env :: var ("_CLAP_IFS") . ok () . and_then (| i | i . parse () . ok ()) ; let mut args = args . clone () ; if args . len () == index { args . push ("" . into ()) ; } let completions = crate :: engine :: complete (cmd , args , index , current_dir) ? ; for (i , candidate) in completions . iter () . enumerate () { if i != 0 { write ! (buf , "{}" , ifs . as_deref () . unwrap_or ("\n")) ? ; } write ! (buf , "{}" , Self :: escape_value (& candidate . get_value () . to_string_lossy ())) ? ; if let Some (help) = candidate . get_help () { write ! (buf , ":{}" , Self :: escape_help (help . to_string () . lines () . next () . unwrap_or_default ())) ? ; } } Ok (()) } }
    };
}

impl_139!()