/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_USE_0001
/* FP:errors.rs-0002 */ use crate :: rustc_complete :: codes :: * ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: { Diag , EmissionGuarantee , Subdiagnostic } ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_USE_0003
/* FP:errors.rs-0006 */ use rustc_macros :: { Diagnostic , Subdiagnostic } ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: lint :: Level ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_USE_0005
/* FP:errors.rs-0010 */ use crate :: rustc_complete :: { Span , Symbol } ;
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_USE_0006
/* FP:errors.rs-0012 */ use crate :: fluent_generated as fluent ;
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_STRUCT_0007
/* FP:errors.rs-0014 */ # [derive (Diagnostic)] # [diag (lint_overruled_attribute , code = E0453)] pub (crate) struct OverruledAttribute < 'a > { # [primary_span] pub span : Span , # [label] pub overruled : Span , pub lint_level : & 'a str , pub lint_source : Symbol , # [subdiagnostic] pub sub : OverruledAttributeSub , }
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_ENUM_0008
/* FP:errors.rs-0016 */ pub (crate) enum OverruledAttributeSub { DefaultSource { id : String } , NodeSource { span : Span , reason : Option < Symbol > } , CommandLineSource , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_IMPL_0009
/* FP:errors.rs-0018 */ impl Subdiagnostic for OverruledAttributeSub { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { match self { OverruledAttributeSub :: DefaultSource { id } => { diag . note (fluent :: lint_default_source) ; diag . arg ("id" , id) ; } OverruledAttributeSub :: NodeSource { span , reason } => { diag . span_label (span , fluent :: lint_node_source) ; if let Some (rationale) = reason { # [allow (rustc :: untranslatable_diagnostic)] diag . note (rationale . to_string ()) ; } } OverruledAttributeSub :: CommandLineSource => { diag . note (fluent :: lint_command_line_source) ; } } } }
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ # [derive (Diagnostic)] # [diag (lint_malformed_attribute , code = E0452)] pub (crate) struct MalformedAttribute { # [primary_span] pub span : Span , # [subdiagnostic] pub sub : MalformedAttributeSub , }
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_ENUM_0011
/* FP:errors.rs-0022 */ # [derive (Subdiagnostic)] pub (crate) enum MalformedAttributeSub { # [label (lint_bad_attribute_argument)] BadAttributeArgument (# [primary_span] Span) , # [label (lint_reason_must_be_string_literal)] ReasonMustBeStringLiteral (# [primary_span] Span) , # [label (lint_reason_must_come_last)] ReasonMustComeLast (# [primary_span] Span) , }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (Diagnostic)] # [diag (lint_unknown_tool_in_scoped_lint , code = E0710)] pub (crate) struct UnknownToolInScopedLint { # [primary_span] pub span : Option < Span > , pub tool_name : Symbol , pub lint_name : String , # [help] pub is_nightly_build : bool , }
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (Diagnostic)] # [diag (lint_builtin_ellipsis_inclusive_range_patterns , code = E0783)] pub (crate) struct BuiltinEllipsisInclusiveRangePatterns { # [primary_span] pub span : Span , # [suggestion (style = "short" , code = "{replace}" , applicability = "machine-applicable")] pub suggestion : Span , pub replace : String , }
/* FP:errors.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_STRUCT_0014
/* FP:errors.rs-0028 */ # [derive (Subdiagnostic)] # [note (lint_requested_level)] pub (crate) struct RequestedLevel < 'a > { pub level : Level , pub lint_name : & 'a str , }
/* FP:errors.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_STRUCT_0015
/* FP:errors.rs-0030 */ # [derive (Diagnostic)] # [diag (lint_unsupported_group , code = E0602)] pub (crate) struct UnsupportedGroup { pub lint_group : String , }
/* FP:errors.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_errors_STRUCT_0016
/* FP:errors.rs-0032 */ # [derive (Diagnostic)] # [diag (lint_check_name_unknown_tool , code = E0602)] pub (crate) struct CheckNameUnknownTool < 'a > { pub tool_name : Symbol , # [subdiagnostic] pub sub : RequestedLevel < 'a > , }