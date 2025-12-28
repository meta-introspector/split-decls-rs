macro_rules! deps {
    () => {
        Generator!();
        Bash!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Generator for Bash { fn file_name (& self , name : & str) -> String { format ! ("{name}.bash") } fn generate (& self , cmd : & Command , buf : & mut dyn Write) { self . try_generate (cmd , buf) . expect ("failed to write completion file") ; } fn try_generate (& self , cmd : & Command , buf : & mut dyn Write) -> Result < () , Error > { let bin_name = cmd . get_bin_name () . expect ("crate::generate should have set the bin_name") ; let fn_name = bin_name . replace ('-' , "__") ; write ! (buf , "_{name}() {{
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ \"${{BASH_VERSINFO[0]}}\" -ge 4 ]]; then
        cur=\"$2\"
    else
        cur=\"${{COMP_WORDS[COMP_CWORD]}}\"
    fi
    prev=\"$3\"
    cmd=\"\"
    opts=\"\"

    for i in \"${{COMP_WORDS[@]:0:COMP_CWORD}}\"
    do
        case \"${{cmd}},${{i}}\" in
            \",$1\")
                cmd=\"{cmd}\"
                ;;{subcmds}
            *)
                ;;
        esac
    done

    case \"${{cmd}}\" in
        {cmd})
            opts=\"{name_opts}\"
            if [[ ${{cur}} == -* || ${{COMP_CWORD}} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
                return 0
            fi
            case \"${{prev}}\" in{name_opts_details}
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{cur}}\") )
            return 0
            ;;{subcmd_details}
    esac
}}

if [[ \"${{BASH_VERSINFO[0]}}\" -eq 4 && \"${{BASH_VERSINFO[1]}}\" -ge 4 || \"${{BASH_VERSINFO[0]}}\" -gt 4 ]]; then
    complete -F _{name} -o nosort -o bashdefault -o default {name}
else
    complete -F _{name} -o bashdefault -o default {name}
fi
" , name = bin_name , cmd = fn_name , name_opts = all_options_for_path (cmd , bin_name) , name_opts_details = option_details_for_path (cmd , bin_name) , subcmds = all_subcommands (cmd , & fn_name) , subcmd_details = subcommand_details (cmd)) } }
    };
}

impl_19!()