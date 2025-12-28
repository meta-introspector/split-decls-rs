macro_rules! RedundantLifetimeArgsLint {
    () => {
        # [derive (LintDiagnostic)] # [diag (hir_analysis_redundant_lifetime_args)] # [note] struct RedundantLifetimeArgsLint < 'tcx > { # [doc = " The lifetime we have found to be redundant."] victim : ty :: Region < 'tcx > , candidate : ty :: Region < 'tcx > , }
    };
}

RedundantLifetimeArgsLint!()