macro_rules! deps {
    () => {
        Elvish!();
        Generator!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Generator for Elvish { fn file_name (& self , name : & str) -> String { format ! ("{name}.elv") } fn generate (& self , cmd : & Command , buf : & mut dyn Write) { self . try_generate (cmd , buf) . expect ("failed to write completion file") ; } fn try_generate (& self , cmd : & Command , buf : & mut dyn Write) -> Result < () , Error > { let bin_name = cmd . get_bin_name () . expect ("crate::generate should have set the bin_name") ; let subcommands_cases = generate_inner (cmd , "") ; write ! (buf , r#"
use builtin;
use str;

set edit:completion:arg-completer[{bin_name}] = {{|@words|
    fn spaces {{|n|
        builtin:repeat $n ' ' | str:join ''
    }}
    fn cand {{|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }}
    var command = '{bin_name}'
    for word $words[1..-1] {{
        if (str:has-prefix $word '-') {{
            break
        }}
        set command = $command';'$word
    }}
    var completions = [{subcommands_cases}
    ]
    $completions[$command]
}}
"# ,) } }
    };
}

impl_27!()