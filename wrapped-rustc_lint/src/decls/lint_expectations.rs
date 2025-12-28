macro_rules! lint_expectations {
    () => {
        fn lint_expectations (tcx : TyCtxt < '_ > , () : ()) -> Vec < (LintExpectationId , LintExpectation) > { let krate = tcx . hir_crate_items (()) ; let mut expectations = Vec :: new () ; for owner in krate . owners () { let lints = tcx . shallow_lint_levels_on (owner) ; expectations . extend_from_slice (& lints . expectations) ; } expectations }
    };
}

lint_expectations!();