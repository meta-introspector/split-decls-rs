macro_rules! deps {
    () => {
        ProcMacroKind!();
        ProcMacroExpander!();
    };
}

macro_rules! ProcMacro {
    () => {
        deps!();
        # [doc = " A loaded proc-macro."] # [derive (Debug , Clone , Eq)] pub struct ProcMacro { # [doc = " The name of the proc macro."] pub name : Symbol , pub kind : ProcMacroKind , # [doc = " The expander handle for this proc macro."] pub expander : sync :: Arc < dyn ProcMacroExpander > , # [doc = " Whether this proc-macro is disabled for early name resolution. Notably, the"] # [doc = " [`Self::expander`] is still usable."] pub disabled : bool , }
    };
}

ProcMacro!();