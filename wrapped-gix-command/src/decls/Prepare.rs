macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! Prepare {
    () => {
        deps!();
        # [doc = " A structure to keep settings to use when invoking a command via [`spawn()`][Prepare::spawn()],"] # [doc = " after creating it with [`prepare()`]."] pub struct Prepare { # [doc = " The command to invoke, either directly or with a shell depending on `use_shell`."] pub command : OsString , # [doc = " Additional information to be passed to the spawned command."] pub context : Option < Context > , # [doc = " The way standard input is configured."] pub stdin : std :: process :: Stdio , # [doc = " The way standard output is configured."] pub stdout : std :: process :: Stdio , # [doc = " The way standard error is configured."] pub stderr : std :: process :: Stdio , # [doc = " The arguments to pass to the process being spawned."] pub args : Vec < OsString > , # [doc = " Environment variables to set for the spawned process."] pub env : Vec < (OsString , OsString) > , # [doc = " If `true`, we will use `shell_program` or `sh` to execute the `command`."] pub use_shell : bool , # [doc = " If `true`, `command` is assumed to be a command or path to the program to execute, and it"] # [doc = " will be shell-quoted to assure it will be executed as is and without splitting across"] # [doc = " whitespace."] pub quote_command : bool , # [doc = " The name or path to the shell program to use instead of `sh`."] pub shell_program : Option < OsString > , # [doc = " If `true` (default `true` on Windows and `false` everywhere else) we will see if it's safe"] # [doc = " to manually invoke `command` after splitting its arguments as a shell would do."] # [doc = ""] # [doc = " Note that outside of Windows, it's generally not advisable as this removes support for"] # [doc = " literal shell scripts with shell-builtins."] # [doc = ""] # [doc = " This mimics the behaviour we see with `git` on Windows, which also won't invoke the shell"] # [doc = " there at all."] # [doc = ""] # [doc = " Only effective if `use_shell` is `true` as well, as the shell will be used as a fallback if"] # [doc = " it's not possible to split arguments as the command-line contains 'scripting'."] pub allow_manual_arg_splitting : bool , }
    };
}

Prepare!();