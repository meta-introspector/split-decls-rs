macro_rules! deps {
    () => {
        Powershell!();
        EnvCompleter!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl EnvCompleter for Powershell { fn name (& self) -> & 'static str { "powershell" } fn is (& self , name : & str) -> bool { name == "powershell" || name == "powershell_ise" } fn write_registration (& self , var : & str , _name : & str , bin : & str , completer : & str , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let bin = shlex :: try_quote (bin) . unwrap_or (std :: borrow :: Cow :: Borrowed (bin)) ; let completer = shlex :: try_quote (completer) . unwrap_or (std :: borrow :: Cow :: Borrowed (completer)) ; writeln ! (buf , r#"
Register-ArgumentCompleter -Native -CommandName {bin} -ScriptBlock {{
    param($wordToComplete, $commandAst, $cursorPosition)

    $prev = $env:{var};
    $env:{var} = "powershell";

    $args = $commandAst.Extent.Text
    $args = $args.Substring(0, [math]::Min($cursorPosition, $args.Length));
    if ($wordToComplete -eq "") {{
        $args += " ''";
    }}

    $results = Invoke-Expression @"
& {completer} -- $args
"@;
    if ($null -eq $prev) {{
        Remove-Item Env:\{var};
    }} else {{
        $env:{var} = $prev;
    }}
    $results | ForEach-Object {{
        $split = $_.Split("`t");
        $cmd = $split[0];

        if ($split.Length -eq 2) {{
            $help = $split[1];
        }}
        else {{
            $help = $split[0];
        }}

        [System.Management.Automation.CompletionResult]::new($cmd, $cmd, 'ParameterValue', $help)
    }}
}};
        "#) } fn write_complete (& self , cmd : & mut clap :: Command , args : Vec < OsString > , current_dir : Option < & std :: path :: Path > , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let index = args . len () - 1 ; let completions = crate :: engine :: complete (cmd , args , index , current_dir) ? ; for candidate in completions { write ! (buf , "{}" , candidate . get_value () . to_string_lossy ()) ? ; if let Some (help) = candidate . get_help () { write ! (buf , "\t{}" , help . to_string () . lines () . next () . unwrap_or_default ()) ? ; } writeln ! (buf) ? ; } Ok (()) } }
    };
}

impl_137!();