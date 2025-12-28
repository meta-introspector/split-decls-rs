macro_rules! deps {
    () => {
        PowerShell!();
        Shell!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Shell { # [doc = " Parse a shell from a path to the executable for the shell"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use clap_complete::shells::Shell;"] # [doc = ""] # [doc = " assert_eq!(Shell::from_shell_path(\"/bin/bash\"), Some(Shell::Bash));"] # [doc = " assert_eq!(Shell::from_shell_path(\"/usr/bin/zsh\"), Some(Shell::Zsh));"] # [doc = " assert_eq!(Shell::from_shell_path(\"/opt/my_custom_shell\"), None);"] # [doc = " ```"] pub fn from_shell_path < P : AsRef < Path > > (path : P) -> Option < Shell > { parse_shell_from_path (path . as_ref ()) } # [doc = " Determine the user's current shell from the environment"] # [doc = ""] # [doc = " This will read the SHELL environment variable and try to determine which shell is in use"] # [doc = " from that."] # [doc = ""] # [doc = " If SHELL is not set, then on windows, it will default to powershell, and on"] # [doc = " other operating systems it will return `None`."] # [doc = ""] # [doc = " If SHELL is set, but contains a value that doesn't correspond to one of the supported shell"] # [doc = " types, then return `None`."] # [doc = ""] # [doc = " # Example:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clap::Command;"] # [doc = " use clap_complete::{generate, shells::Shell};"] # [doc = " # fn build_cli() -> Command {"] # [doc = " #     Command::new(\"compl\")"] # [doc = " # }"] # [doc = " let mut cmd = build_cli();"] # [doc = " generate(Shell::from_env().unwrap_or(Shell::Bash), &mut cmd, \"myapp\", &mut std::io::stdout());"] # [doc = " ```"] pub fn from_env () -> Option < Shell > { if let Some (env_shell) = std :: env :: var_os ("SHELL") { Shell :: from_shell_path (env_shell) } else if cfg ! (windows) { Some (Shell :: PowerShell) } else { None } } }
    };
}

impl_53!();