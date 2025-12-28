macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! SelectCppBuildToolWorkload {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_select_cpp_build_tool_workload)] pub (crate) struct SelectCppBuildToolWorkload ;
    };
}

SelectCppBuildToolWorkload!()