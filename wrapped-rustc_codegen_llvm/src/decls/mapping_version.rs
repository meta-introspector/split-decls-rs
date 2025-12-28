macro_rules! mapping_version {
    () => {
        # [doc = " Returns LLVM's `coverage::CovMapVersion::CurrentVersion` (CoverageMapping.h)"] # [doc = " as a raw numeric value. For historical reasons, the numeric value is 1 less"] # [doc = " than the number in the version's name, so `Version7` is actually `6u32`."] pub (crate) fn mapping_version () -> u32 { unsafe { llvm :: LLVMRustCoverageMappingVersion () } }
    };
}

mapping_version!()