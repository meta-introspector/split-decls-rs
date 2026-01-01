/* FP:decorate_diag.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_USE_0001
/* FP:decorate_diag.rs-0002 */ # [doc = " This module provides types and traits for buffering lints until later in compilation."] use crate :: rustc_complete :: node_id :: NodeId ;
/* FP:decorate_diag.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_USE_0002
/* FP:decorate_diag.rs-0004 */ use crate :: rustc_data_structures :: fx :: FxIndexMap ;
/* FP:decorate_diag.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_USE_0003
/* FP:decorate_diag.rs-0006 */ use crate :: rustc_error_messages :: MultiSpan ;
/* FP:decorate_diag.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_USE_0004
/* FP:decorate_diag.rs-0008 */ use crate :: rustc_lint_defs :: { BuiltinLintDiag , Lint , LintId } ;
/* FP:decorate_diag.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_USE_0005
/* FP:decorate_diag.rs-0010 */ use crate :: { DynSend , LintDiagnostic , LintDiagnosticBox } ;
/* FP:decorate_diag.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_ENUM_0006
/* FP:decorate_diag.rs-0012 */ # [doc = " We can't implement `LintDiagnostic` for `BuiltinLintDiag`, because decorating some of its"] # [doc = " variants requires types we don't have yet. So, handle that case separately."] pub enum DecorateDiagCompat { Dynamic (Box < dyn for < 'a > LintDiagnosticBox < 'a , () > + DynSend + 'static >) , Builtin (BuiltinLintDiag) , }
/* FP:decorate_diag.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_IMPL_0007
/* FP:decorate_diag.rs-0014 */ impl std :: fmt :: Debug for DecorateDiagCompat { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("DecorateDiagCompat") . finish () } }
/* FP:decorate_diag.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_IMPL_0008
/* FP:decorate_diag.rs-0016 */ impl ! LintDiagnostic < '_ , () > for BuiltinLintDiag { }
/* FP:decorate_diag.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_IMPL_0009
/* FP:decorate_diag.rs-0018 */ impl < D : for < 'a > LintDiagnostic < 'a , () > + DynSend + 'static > From < D > for DecorateDiagCompat { # [inline] fn from (d : D) -> Self { Self :: Dynamic (Box :: new (d)) } }
/* FP:decorate_diag.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_IMPL_0010
/* FP:decorate_diag.rs-0020 */ impl From < BuiltinLintDiag > for DecorateDiagCompat { # [inline] fn from (b : BuiltinLintDiag) -> Self { Self :: Builtin (b) } }
/* FP:decorate_diag.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_STRUCT_0011
/* FP:decorate_diag.rs-0022 */ # [doc = " Lints that are buffered up early on in the `Session` before the"] # [doc = " `LintLevels` is calculated."] # [derive (Debug)] pub struct BufferedEarlyLint { # [doc = " The span of code that we are linting on."] pub span : Option < MultiSpan > , # [doc = " The `NodeId` of the AST node that generated the lint."] pub node_id : NodeId , # [doc = " A lint Id that can be passed to"] # [doc = " `crate::rustc_lint::early::EarlyContextAndPass::check_id`."] pub lint_id : LintId , # [doc = " Customization of the `Diag<'_>` for the lint."] pub diagnostic : DecorateDiagCompat , }
/* FP:decorate_diag.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_STRUCT_0012
/* FP:decorate_diag.rs-0024 */ # [derive (Default , Debug)] pub struct LintBuffer { pub map : FxIndexMap < NodeId , Vec < BufferedEarlyLint > > , }
/* FP:decorate_diag.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_decorate_diag_IMPL_0013
/* FP:decorate_diag.rs-0026 */ impl LintBuffer { pub fn add_early_lint (& mut self , early_lint : BufferedEarlyLint) { self . map . entry (early_lint . node_id) . or_default () . push (early_lint) ; } pub fn take (& mut self , id : NodeId) -> Vec < BufferedEarlyLint > { self . map . swap_remove (& id) . unwrap_or_default () } pub fn buffer_lint (& mut self , lint : & 'static Lint , node_id : NodeId , span : impl Into < MultiSpan > , decorate : impl Into < DecorateDiagCompat > ,) { self . add_early_lint (BufferedEarlyLint { lint_id : LintId :: of (lint) , node_id , span : Some (span . into ()) , diagnostic : decorate . into () , }) ; } }