macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! StaticLibraryNativeArtifacts {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_static_library_native_artifacts)] pub (crate) struct StaticLibraryNativeArtifacts ;
    };
}

StaticLibraryNativeArtifacts!()