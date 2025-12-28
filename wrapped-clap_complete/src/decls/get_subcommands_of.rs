macro_rules! get_subcommands_of {
    () => {
        fn get_subcommands_of (parent : & Command) -> String { debug ! ("get_subcommands_of: Has subcommands...{:?}" , parent . has_subcommands ()) ; if ! parent . has_subcommands () { return String :: new () ; } let subcommand_names = utils :: subcommands (parent) ; let mut all_subcommands = vec ! [] ; for (name , bin_name) in & subcommand_names { debug ! ("get_subcommands_of:iter: parent={}, name={name}, bin_name={bin_name}" , parent . get_name () ,) ; let mut segments = vec ! [format ! ("({name})")] ; let subcommand_args = get_args_of (parser_of (parent , bin_name) . expect (INTERNAL_ERROR_MSG) , Some (parent) ,) ; if ! subcommand_args . is_empty () { segments . push (subcommand_args) ; } let children = get_subcommands_of (parser_of (parent , bin_name) . expect (INTERNAL_ERROR_MSG)) ; if ! children . is_empty () { segments . push (children) ; } segments . push (String :: from (";;")) ; all_subcommands . push (segments . join ("\n")) ; } let parent_bin_name = parent . get_bin_name () . expect ("crate::generate should have set the bin_name") ; format ! ("
    case $state in
    ({name})
        words=($line[{pos}] \"${{words[@]}}\")
        (( CURRENT += 1 ))
        curcontext=\"${{curcontext%:*:*}}:{name_hyphen}-command-$line[{pos}]:\"
        case $line[{pos}] in
            {subcommands}
        esac
    ;;
esac" , name = parent . get_name () , name_hyphen = parent_bin_name . replace (' ' , "-") , subcommands = all_subcommands . join ("\n") , pos = parent . get_positionals () . count () + 1) }
    };
}

get_subcommands_of!()