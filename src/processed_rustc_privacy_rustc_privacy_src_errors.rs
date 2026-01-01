/* FP:errors.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_USE_0001
/* FP:errors.rs-0002 */ use crate :: rustc_complete :: codes :: * ;
/* FP:errors.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_USE_0002
/* FP:errors.rs-0004 */ use crate :: rustc_complete :: { DiagArgFromDisplay , MultiSpan } ;
/* FP:errors.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_USE_0003
/* FP:errors.rs-0006 */ use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;
/* FP:errors.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_USE_0004
/* FP:errors.rs-0008 */ use crate :: rustc_complete :: { Span , Symbol } ;
/* FP:errors.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_STRUCT_0005
/* FP:errors.rs-0010 */ # [derive (Diagnostic)] # [diag (privacy_field_is_private , code = E0451)] pub (crate) struct FieldIsPrivate { # [primary_span] pub span : MultiSpan , # [label] pub struct_span : Option < Span > , pub field_names : String , pub variant_descr : & 'static str , pub def_path_str : String , # [subdiagnostic] pub labels : Vec < FieldIsPrivateLabel > , pub len : usize , }
/* FP:errors.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_ENUM_0006
/* FP:errors.rs-0012 */ # [derive (Subdiagnostic)] pub (crate) enum FieldIsPrivateLabel { # [label (privacy_field_is_private_is_update_syntax_label)] IsUpdateSyntax { # [primary_span] span : Span , rest_field_names : String , rest_len : usize , } , # [label (privacy_field_is_private_label)] Other { # [primary_span] span : Span , } , }
/* FP:errors.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_STRUCT_0007
/* FP:errors.rs-0014 */ # [derive (Diagnostic)] # [diag (privacy_item_is_private)] pub (crate) struct ItemIsPrivate < 'a > { # [primary_span] # [label] pub span : Span , pub kind : & 'a str , pub descr : DiagArgFromDisplay < 'a > , }
/* FP:errors.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_STRUCT_0008
/* FP:errors.rs-0016 */ # [derive (Diagnostic)] # [diag (privacy_unnamed_item_is_private)] pub (crate) struct UnnamedItemIsPrivate { # [primary_span] pub span : Span , pub kind : & 'static str , }
/* FP:errors.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_STRUCT_0009
/* FP:errors.rs-0018 */ # [derive (Diagnostic)] # [diag (privacy_in_public_interface , code = E0446)] pub (crate) struct InPublicInterface < 'a > { # [primary_span] # [label] pub span : Span , pub vis_descr : & 'static str , pub kind : & 'a str , pub descr : DiagArgFromDisplay < 'a > , # [label (privacy_visibility_label)] pub vis_span : Span , }
/* FP:errors.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_STRUCT_0010
/* FP:errors.rs-0020 */ # [derive (Diagnostic)] # [diag (privacy_report_effective_visibility)] pub (crate) struct ReportEffectiveVisibility { # [primary_span] pub span : Span , pub descr : String , }
/* FP:errors.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_STRUCT_0011
/* FP:errors.rs-0022 */ # [derive (LintDiagnostic)] # [diag (privacy_from_private_dep_in_public_interface)] pub (crate) struct FromPrivateDependencyInPublicInterface < 'a > { pub kind : & 'a str , pub descr : DiagArgFromDisplay < 'a > , pub krate : Symbol , }
/* FP:errors.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_STRUCT_0012
/* FP:errors.rs-0024 */ # [derive (LintDiagnostic)] # [diag (privacy_unnameable_types_lint)] pub (crate) struct UnnameableTypesLint < 'a > { # [label] pub span : Span , pub kind : & 'a str , pub descr : DiagArgFromDisplay < 'a > , pub reachable_vis : & 'a str , pub reexported_vis : & 'a str , }
/* FP:errors.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_privacy_src_errors_STRUCT_0013
/* FP:errors.rs-0026 */ # [derive (LintDiagnostic)] # [diag (privacy_private_interface_or_bounds_lint)] pub (crate) struct PrivateInterfacesOrBoundsLint < 'a > { # [label (privacy_item_label)] pub item_span : Span , pub item_kind : & 'a str , pub item_descr : DiagArgFromDisplay < 'a > , pub item_vis_descr : & 'a str , # [note (privacy_ty_note)] pub ty_span : Span , pub ty_kind : & 'a str , pub ty_descr : DiagArgFromDisplay < 'a > , pub ty_vis_descr : & 'a str , }