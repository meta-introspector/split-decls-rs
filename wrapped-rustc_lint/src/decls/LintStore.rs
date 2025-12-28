macro_rules! deps {
    () => {
        TargetLint!();
        EarlyLintPassFactory!();
        LintGroup!();
        LateLintPassFactory!();
    };
}

macro_rules! LintStore {
    () => {
        deps!();
        # [doc = " Information about the registered lints."] pub struct LintStore { # [doc = " Registered lints."] lints : Vec < & 'static Lint > , # [doc = " Constructor functions for each variety of lint pass."] # [doc = ""] # [doc = " These should only be called once, but since we want to avoid locks or"] # [doc = " interior mutability, we don't enforce this (and lints should, in theory,"] # [doc = " be compatible with being constructed more than once, though not"] # [doc = " necessarily in a sane manner. This is safe though.)"] pub pre_expansion_passes : Vec < Box < EarlyLintPassFactory > > , pub early_passes : Vec < Box < EarlyLintPassFactory > > , pub late_passes : Vec < Box < LateLintPassFactory > > , # [doc = " This is unique in that we construct them per-module, so not once."] pub late_module_passes : Vec < Box < LateLintPassFactory > > , # [doc = " Lints indexed by name."] by_name : UnordMap < String , TargetLint > , # [doc = " Map of registered lint groups to what lints they expand to."] lint_groups : FxIndexMap < & 'static str , LintGroup > , }
    };
}

LintStore!()