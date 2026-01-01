/* FP:ident.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0001
/* FP:ident.rs-0002 */ use Determinacy :: * ;
/* FP:ident.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0002
/* FP:ident.rs-0004 */ use Namespace :: * ;
/* FP:ident.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0003
/* FP:ident.rs-0006 */ use crate :: rustc_complete :: { self as ast , NodeId } ;
/* FP:ident.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0004
/* FP:ident.rs-0008 */ use crate :: rustc_complete :: ErrorGuaranteed ;
/* FP:ident.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0005
/* FP:ident.rs-0010 */ use crate :: rustc_complete :: def :: { DefKind , MacroKinds , Namespace , NonMacroAttrKind , PartialRes , PerNS } ;
/* FP:ident.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0006
/* FP:ident.rs-0012 */ use crate :: rustc_complete :: bug ;
/* FP:ident.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0007
/* FP:ident.rs-0014 */ use crate :: rustc_complete :: lint :: BuiltinLintDiag ;
/* FP:ident.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0008
/* FP:ident.rs-0016 */ use crate :: rustc_complete :: lint :: builtin :: PROC_MACRO_DERIVE_RESOLUTION_FALLBACK ;
/* FP:ident.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0009
/* FP:ident.rs-0018 */ use crate :: rustc_complete :: parse :: feature_err ;
/* FP:ident.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0010
/* FP:ident.rs-0020 */ use crate :: rustc_complete :: hygiene :: { ExpnId , ExpnKind , LocalExpnId , MacroKind , SyntaxContext } ;
/* FP:ident.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0011
/* FP:ident.rs-0022 */ use crate :: rustc_complete :: { Ident , Span , kw , sym } ;
/* FP:ident.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0012
/* FP:ident.rs-0024 */ use tracing :: { debug , instrument } ;
/* FP:ident.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0013
/* FP:ident.rs-0026 */ use crate :: errors :: { ParamKindInEnumDiscriminant , ParamKindInNonTrivialAnonConst } ;
/* FP:ident.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0014
/* FP:ident.rs-0028 */ use crate :: imports :: { Import , NameResolution } ;
/* FP:ident.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0015
/* FP:ident.rs-0030 */ use crate :: late :: { ConstantHasGenerics , NoConstantGenericsReason , PathSource , Rib , RibKind } ;
/* FP:ident.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0016
/* FP:ident.rs-0032 */ use crate :: macros :: { MacroRulesScope , sub_namespace_match } ;
/* FP:ident.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_USE_0017
/* FP:ident.rs-0034 */ use crate :: { AmbiguityError , AmbiguityErrorMisc , AmbiguityKind , BindingKey , CmResolver , Determinacy , Finalize , ImportKind , LexicalScopeBinding , Module , ModuleKind , ModuleOrUniformRoot , NameBinding , NameBindingKind , ParentScope , PathResult , PrivacyError , Res , ResolutionError , Resolver , Scope , ScopeSet , Segment , Stage , Used , Weak , errors , } ;
/* FP:ident.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_ENUM_0018
/* FP:ident.rs-0036 */ # [derive (Copy , Clone)] pub enum UsePrelude { No , Yes , }
/* FP:ident.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_IMPL_0019
/* FP:ident.rs-0038 */ impl From < UsePrelude > for bool { fn from (up : UsePrelude) -> bool { matches ! (up , UsePrelude :: Yes) } }
/* FP:ident.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_ENUM_0020
/* FP:ident.rs-0040 */ # [derive (Debug , PartialEq , Clone , Copy)] enum Shadowing { Restricted , Unrestricted , }
/* FP:ident.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_ident_IMPL_0021