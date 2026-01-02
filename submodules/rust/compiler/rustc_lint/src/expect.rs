mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_middle :: lint :: LintExpectation ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: lint :: LintExpectationId ;}
mkuse!{use rustc_session :: lint :: builtin :: UNFULFILLED_LINT_EXPECTATIONS ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use crate :: lints :: { Expectation , ExpectationNote } ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { lint_expectations , check_expectations , .. * providers } ; }
}

macro_rules! lint_expectations_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lint_expectations in module {}", module_path!());
    };
}

mkfn!{
    lint_expectations_introspect!();
    fn lint_expectations (tcx : TyCtxt < '_ > , () : ()) -> Vec < (LintExpectationId , LintExpectation) > { let krate = tcx . hir_crate_items (()) ; let mut expectations = Vec :: new () ; for owner in krate . owners () { let lints = tcx . shallow_lint_levels_on (owner) ; expectations . extend_from_slice (& lints . expectations) ; } expectations }
}

macro_rules! check_expectations_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_expectations in module {}", module_path!());
    };
}

mkfn!{
    check_expectations_introspect!();
    fn check_expectations (tcx : TyCtxt < '_ > , tool_filter : Option < Symbol >) { let lint_expectations = tcx . lint_expectations (()) ; let fulfilled_expectations = tcx . dcx () . steal_fulfilled_expectation_ids () ; let canonicalize_id = | expect_id : & LintExpectationId | { match * expect_id { LintExpectationId :: Unstable { attr_id , lint_index : Some (lint_index) } => { (attr_id , lint_index) } LintExpectationId :: Stable { hir_id , attr_index , lint_index : Some (lint_index) } => { let attr_id = tcx . hir_attrs (hir_id) [attr_index as usize] . id () ; (attr_id , lint_index) } _ => panic ! ("fulfilled expectations must have a lint index") , } } ; let fulfilled_expectations : FxHashSet < _ > = fulfilled_expectations . iter () . map (canonicalize_id) . collect () ; for (expect_id , expectation) in lint_expectations { let LintExpectationId :: Stable { hir_id , .. } = expect_id else { unreachable ! ("at this stage all `LintExpectationId`s are stable") ; } ; let expect_id = canonicalize_id (expect_id) ; if ! fulfilled_expectations . contains (& expect_id) && tool_filter . is_none_or (| filter | expectation . lint_tool == Some (filter)) { let rationale = expectation . reason . map (| rationale | ExpectationNote { rationale }) ; let note = expectation . is_unfulfilled_lint_expectations ; tcx . emit_node_span_lint (UNFULFILLED_LINT_EXPECTATIONS , * hir_id , expectation . emission_span , Expectation { rationale , note } ,) ; } } }
}