macro_rules! LinkerOutput {
    () => {
        # [derive (LintDiagnostic)] # [diag (codegen_ssa_linker_output)] # [doc = " Translating this is kind of useless. We don't pass translation flags to the linker, so we'd just"] # [doc = " end up with inconsistent languages within the same diagnostic."] struct LinkerOutput { inner : String , }
    };
}

LinkerOutput!();