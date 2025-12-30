// Generated macro for CommandExt (trait)
macro_rules! Depcrate_windowsCommandExt {
() => {
// Module: crate::windows
// Provides: {"CommandExt"}
// Dependencies: {}
# [doc = " Windows-specific extensions to the [`Command`] builder."] # [doc = ""] # [doc = " This trait is sealed: it cannot be implemented outside `async-process`."] # [doc = " This is so that future additional methods are not breaking changes."] pub trait CommandExt : crate :: sealed :: Sealed { # [doc = " Sets the [process creation flags][1] to be passed to `CreateProcess`."] # [doc = ""] # [doc = " These will always be ORed with `CREATE_UNICODE_ENVIRONMENT`."] # [doc = ""] # [doc = " [1]: https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags"] fn creation_flags (& mut self , flags : u32) -> & mut Command ; # [doc = " Append literal text to the command line without any quoting or escaping."] # [doc = ""] # [doc = " This is useful for passing arguments to `cmd.exe /c`, which doesn't follow"] # [doc = " `CommandLineToArgvW` escaping rules."] fn raw_arg < S : AsRef < OsStr > > (& mut self , text_to_append_as_is : S) -> & mut Command ; }
};
}
