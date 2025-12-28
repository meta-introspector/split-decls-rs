macro_rules! should_do_rust_2021_incompatible_closure_captures_analysis {
    () => {
        # [instrument (level = "debug" , skip (tcx))] fn should_do_rust_2021_incompatible_closure_captures_analysis (tcx : TyCtxt < '_ > , closure_id : HirId ,) -> bool { if tcx . sess . at_least_rust_2021 () { return false ; } let level = tcx . lint_level_at_node (lint :: builtin :: RUST_2021_INCOMPATIBLE_CLOSURE_CAPTURES , closure_id) . level ; ! matches ! (level , lint :: Level :: Allow) }
    };
}

should_do_rust_2021_incompatible_closure_captures_analysis!();