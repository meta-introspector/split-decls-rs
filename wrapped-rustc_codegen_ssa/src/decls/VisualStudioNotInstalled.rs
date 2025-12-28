macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! VisualStudioNotInstalled {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_visual_studio_not_installed)] pub (crate) struct VisualStudioNotInstalled ;
    };
}

VisualStudioNotInstalled!();