macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! CheckInstalledVisualStudio {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_check_installed_visual_studio)] pub (crate) struct CheckInstalledVisualStudio ;
    };
}

CheckInstalledVisualStudio!();