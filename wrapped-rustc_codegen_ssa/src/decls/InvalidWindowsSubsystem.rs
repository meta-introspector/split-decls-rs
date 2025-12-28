macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! InvalidWindowsSubsystem {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_invalid_windows_subsystem)] pub (crate) struct InvalidWindowsSubsystem { pub subsystem : Symbol , }
    };
}

InvalidWindowsSubsystem!();