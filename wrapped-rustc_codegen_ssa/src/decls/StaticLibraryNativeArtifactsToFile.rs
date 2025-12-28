macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! StaticLibraryNativeArtifactsToFile {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_static_library_native_artifacts_to_file)] pub (crate) struct StaticLibraryNativeArtifactsToFile < 'a > { pub path : & 'a Path , }
    };
}

StaticLibraryNativeArtifactsToFile!()