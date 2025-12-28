macro_rules! deps {
    () => {
        Generator!();
        PowerShell!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Generator for PowerShell { fn file_name (& self , name : & str) -> String { format ! ("_{name}.ps1") } fn generate (& self , cmd : & Command , buf : & mut dyn Write) { self . try_generate (cmd , buf) . expect ("failed to write completion file") ; } fn try_generate (& self , cmd : & Command , buf : & mut dyn Write) -> Result < () , Error > { let bin_name = cmd . get_bin_name () . expect ("crate::generate should have set the bin_name") ; let subcommands_cases = generate_inner (cmd , "") ; write ! (buf , r#"
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName '{bin_name}' -ScriptBlock {{
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        '{bin_name}'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {{
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {{
                break
        }}
        $element.Value
    }}) -join ';'

    $completions = @(switch ($command) {{{subcommands_cases}
    }})

    $completions.Where{{ $_.CompletionText -like "$wordToComplete*" }} |
        Sort-Object -Property ListItemText
}}
"#) } }
    };
}

impl_42!()