/* FP:expect.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_USE_0001
/* FP:expect.rs-0002 */ use crate :: rustc_data_structures :: fx :: FxHashSet ;
/* FP:expect.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_USE_0002
/* FP:expect.rs-0004 */ use crate :: rustc_complete :: lint :: LintExpectation ;
/* FP:expect.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_USE_0003
/* FP:expect.rs-0006 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:expect.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_USE_0004
/* FP:expect.rs-0008 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:expect.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_USE_0005
/* FP:expect.rs-0010 */ use crate :: rustc_complete :: lint :: LintExpectationId ;
/* FP:expect.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_USE_0006
/* FP:expect.rs-0012 */ use crate :: rustc_complete :: lint :: builtin :: UNFULFILLED_LINT_EXPECTATIONS ;
/* FP:expect.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_USE_0007
/* FP:expect.rs-0014 */ use crate :: rustc_complete :: Symbol ;
/* FP:expect.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_USE_0008
/* FP:expect.rs-0016 */ use crate :: lints :: { Expectation , ExpectationNote } ;
/* FP:expect.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_FN_0009
/* FP:expect.rs-0018 */ pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { lint_expectations , check_expectations , .. * providers } ; }
/* FP:expect.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_FN_0010
/* FP:expect.rs-0020 */ fn lint_expectations (tcx : TyCtxt < '_ > , () : ()) -> Vec < (LintExpectationId , LintExpectation) > { let krate = tcx . hir_crate_items (()) ; let mut expectations = Vec :: new () ; for owner in krate . owners () { let lints = tcx . shallow_lint_levels_on (owner) ; expectations . extend_from_slice (& lints . expectations) ; } expectations }
/* FP:expect.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_expect_FN_0011
/* FP:expect.rs-0022 */ fn check_expectations (tcx : TyCtxt < '_ > , tool_filter : Option < Symbol >) { let lint_expectations = tcx . lint_expectations (()) ; let fulfilled_expectations = tcx . dcx () . steal_fulfilled_expectation_ids () ; let canonicalize_id = | expect_id : & LintExpectationId | { match * expect_id { LintExpectationId :: Unstable { attr_id , lint_index : Some (lint_index) } => { (attr_id , lint_index) } LintExpectationId :: Stable { hir_id , attr_index , lint_index : Some (lint_index) } => { let attr_id = tcx . hir_attrs (hir_id) [attr_index as usize] . id () ; (attr_id , lint_index) } _ => panic ! ("fulfilled expectations must have a lint index") , } } ; let fulfilled_expectations : FxHashSet < _ > = fulfilled_expectations . iter () . map (canonicalize_id) . collect () ; for (expect_id , expectation) in lint_expectations { let LintExpectationId :: Stable { hir_id , .. } = expect_id else { unreachable ! ("at this stage all `LintExpectationId`s are stable") ; } ; let expect_id = canonicalize_id (expect_id) ; if ! fulfilled_expectations . contains (& expect_id) && tool_filter . is_none_or (| filter | expectation . lint_tool == Some (filter)) { let rationale = expectation . reason . map (| rationale | ExpectationNote { rationale }) ; let note = expectation . is_unfulfilled_lint_expectations ; tcx . emit_node_span_lint (UNFULFILLED_LINT_EXPECTATIONS , * hir_id , expectation . emission_span , Expectation { rationale , note } ,) ; } } }