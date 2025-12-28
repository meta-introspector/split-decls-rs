macro_rules! ProcMacroLibrary {
    () => {
        struct ProcMacroLibrary { proc_macros : & 'static ProcMacros , _lib : Library , }
    };
}

ProcMacroLibrary!();