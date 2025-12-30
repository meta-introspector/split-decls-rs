// Generated macro for impl_204 (impl)
macro_rules! Depcrate_env_shellsimpl_204 {
() => {
// Module: crate::env::shells
// Provides: {"impl_204"}
// Dependencies: {}
impl EnvCompleter for Bash { fn name (& self) -> & 'static str { "bash" } fn is (& self , name : & str) -> bool { name == "bash" } fn write_registration (& self , var : & str , name : & str , bin : & str , completer : & str , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let escaped_name = name . replace ('-' , "_") ; let completer = shlex :: try_quote (completer) . unwrap_or (std :: borrow :: Cow :: Borrowed (completer)) ; let script = r#"
_clap_complete_NAME() {
    local IFS=$'\013'
    local _CLAP_COMPLETE_INDEX=${COMP_CWORD}
    local _CLAP_COMPLETE_COMP_TYPE=${COMP_TYPE}
    if compopt +o nospace 2> /dev/null; then
        local _CLAP_COMPLETE_SPACE=false
    else
        local _CLAP_COMPLETE_SPACE=true
    fi
    local words=("${COMP_WORDS[@]}")
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        words[COMP_CWORD]="$2"
    fi
    COMPREPLY=( $( \
        _CLAP_IFS="$IFS" \
        _CLAP_COMPLETE_INDEX="$_CLAP_COMPLETE_INDEX" \
        _CLAP_COMPLETE_COMP_TYPE="$_CLAP_COMPLETE_COMP_TYPE" \
        _CLAP_COMPLETE_SPACE="$_CLAP_COMPLETE_SPACE" \
        VAR="bash" \
        "COMPLETER" -- "${words[@]}" \
    ) )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    elif [[ $_CLAP_COMPLETE_SPACE == false ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
        compopt -o nospace
    fi
}
if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -o nospace -o bashdefault -o nosort -F _clap_complete_NAME BIN
else
    complete -o nospace -o bashdefault -F _clap_complete_NAME BIN
fi
"# . replace ("NAME" , & escaped_name) . replace ("BIN" , bin) . replace ("COMPLETER" , & completer) . replace ("VAR" , var) ; writeln ! (buf , "{script}") ? ; Ok (()) } fn write_complete (& self , cmd : & mut clap :: Command , args : Vec < OsString > , current_dir : Option < & std :: path :: Path > , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let index : usize = std :: env :: var ("_CLAP_COMPLETE_INDEX") . ok () . and_then (| i | i . parse () . ok ()) . unwrap_or_default () ; let _comp_type : CompType = std :: env :: var ("_CLAP_COMPLETE_COMP_TYPE") . ok () . and_then (| i | i . parse () . ok ()) . unwrap_or_default () ; let _space : Option < bool > = std :: env :: var ("_CLAP_COMPLETE_SPACE") . ok () . and_then (| i | i . parse () . ok ()) ; let ifs : Option < String > = std :: env :: var ("_CLAP_IFS") . ok () . and_then (| i | i . parse () . ok ()) ; let completions = crate :: engine :: complete (cmd , args , index , current_dir) ? ; for (i , candidate) in completions . iter () . enumerate () { if i != 0 { write ! (buf , "{}" , ifs . as_deref () . unwrap_or ("\n")) ? ; } write ! (buf , "{}" , candidate . get_value () . to_string_lossy ()) ? ; } Ok (()) } }
};
}
