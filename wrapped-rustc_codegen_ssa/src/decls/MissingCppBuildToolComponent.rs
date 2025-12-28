macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! MissingCppBuildToolComponent {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_missing_cpp_build_tool_component)] pub (crate) struct MissingCppBuildToolComponent ;
    };
}

MissingCppBuildToolComponent!();