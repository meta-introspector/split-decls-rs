macro_rules! deps {
    () => {
        ProcMacro!();
    };
}

macro_rules! CustomProcMacroExpander {
    () => {
        deps!();
        # [doc = " A custom proc-macro expander handle. This handle together with its crate resolves to a [`ProcMacro`]"] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] pub struct CustomProcMacroExpander { proc_macro_id : u32 , }
    };
}

CustomProcMacroExpander!()