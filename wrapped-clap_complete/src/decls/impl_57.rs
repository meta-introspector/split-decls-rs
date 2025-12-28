macro_rules! deps {
    () => {
        Generator!();
        Zsh!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl Generator for Zsh { fn file_name (& self , name : & str) -> String { format ! ("_{name}") } fn generate (& self , cmd : & Command , buf : & mut dyn Write) { self . try_generate (cmd , buf) . expect ("failed to write completion file") ; } fn try_generate (& self , cmd : & Command , buf : & mut dyn Write) -> Result < () , Error > { let bin_name = cmd . get_bin_name () . expect ("crate::generate should have set the bin_name") ; write ! (buf , "#compdef {name}

autoload -U is-at-least

_{name}() {{
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext=\"$curcontext\" state line
    {initial_args}{subcommands}
}}

{subcommand_details}

if [ \"$funcstack[1]\" = \"_{name}\" ]; then
    _{name} \"$@\"
else
    compdef _{name} {name}
fi
" , name = bin_name , initial_args = get_args_of (cmd , None) , subcommands = get_subcommands_of (cmd) , subcommand_details = subcommand_details (cmd)) } }
    };
}

impl_57!()