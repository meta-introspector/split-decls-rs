macro_rules! Error {
    () => {
        # [doc = " The error returned by [ask()][crate::ask()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Terminal prompts are disabled")] Disabled , # [error ("The current platform has no implementation for prompting in the terminal")] UnsupportedPlatform , # [error ("Failed to open terminal at {:?} for writing prompt, or to write it" , crate :: unix :: TTY_PATH)] TtyIo (# [from] std :: io :: Error) , # [cfg (unix)] # [error ("Failed to obtain or set terminal configuration")] TerminalConfiguration (# [from] rustix :: io :: Errno) , }
    };
}

Error!()