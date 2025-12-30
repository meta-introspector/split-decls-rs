// Generated macro for subcommand_details (function)
macro_rules! Depcrate_aot_shells_bashsubcommand_details {
() => {
// Module: crate::aot::shells::bash
// Provides: {"subcommand_details"}
// Dependencies: {}
fn subcommand_details (cmd : & Command) -> String { debug ! ("subcommand_details") ; let mut subcmd_dets = vec ! [String :: new ()] ; let mut scs = utils :: all_subcommands (cmd) . iter () . map (| x | x . 1 . replace (' ' , "__")) . collect :: < Vec < _ > > () ; scs . sort () ; scs . dedup () ; subcmd_dets . extend (scs . iter () . map (| sc | { format ! ("{subcmd})
            opts=\"{sc_opts}\"
            if [[ ${{cur}} == -* || ${{COMP_CWORD}} -eq {level} ]] ; then
                COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
                return 0
            fi
            case \"${{prev}}\" in{opts_details}
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
            return 0
            ;;" , subcmd = sc . replace ('-' , "__") , sc_opts = all_options_for_path (cmd , sc) , level = sc . split ("__") . map (| _ | 1) . sum ::< u64 > () , opts_details = option_details_for_path (cmd , sc)) })) ; subcmd_dets . join ("\n        ") }
};
}
