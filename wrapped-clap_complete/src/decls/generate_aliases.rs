macro_rules! generate_aliases {
    () => {
        fn generate_aliases (completions : & mut String , preamble : & String , arg : & Arg) { use std :: fmt :: Write as _ ; if let Some (aliases) = arg . get_short_and_visible_aliases () { let tooltip = escape_help (arg . get_help () , aliases [0]) ; for alias in aliases { let _ = write ! (completions , "{preamble}'-{alias}', '-{alias}{}', [CompletionResultType]::ParameterName, '{tooltip}')" , if alias . is_uppercase () { " " } else { "" } ,) ; } } if let Some (aliases) = arg . get_long_and_visible_aliases () { let tooltip = escape_help (arg . get_help () , aliases [0]) ; for alias in aliases { let _ = write ! (completions , "{preamble}'--{alias}', '--{alias}', [CompletionResultType]::ParameterName, '{tooltip}')") ; } } }
    };
}

generate_aliases!()