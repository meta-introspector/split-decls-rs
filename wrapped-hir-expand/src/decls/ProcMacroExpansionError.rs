macro_rules! ProcMacroExpansionError {
    () => {
        # [derive (Debug)] pub enum ProcMacroExpansionError { # [doc = " The proc-macro panicked."] Panic (String) , # [doc = " The server itself errored out."] System (String) , }
    };
}

ProcMacroExpansionError!()