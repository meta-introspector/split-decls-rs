/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_USE_0001
/* FP:mod.rs-0002 */ # [allow (rustc :: usage_of_qualified_ty)] use std :: marker :: PhantomData ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_USE_0002
/* FP:mod.rs-0004 */ use crate :: rustc_abi :: HasDataLayout ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: rustc_complete :: ty ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_USE_0004
/* FP:mod.rs-0008 */ use crate :: rustc_complete :: ty :: layout :: { FnAbiOfHelpers , HasTyCtxt , HasTypingEnv , LayoutOfHelpers } ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_USE_0005
/* FP:mod.rs-0010 */ use crate :: rustc_complete :: ty :: { Ty , TyCtxt } ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_USE_0006
/* FP:mod.rs-0012 */ use crate :: { Bridge , Error } ;
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_MOD_0007
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_MOD_0008
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_USE_0009
/* FP:mod.rs-0018 */ pub use helpers :: * ;
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_STRUCT_0010
/* FP:mod.rs-0020 */ # [doc = " Provides direct access to rustc's internal queries."] # [doc = ""] # [doc = " `CompilerInterface` must go through"] # [doc = " this context to obtain internal information."] pub struct CompilerCtxt < 'tcx , B : Bridge > { pub tcx : TyCtxt < 'tcx > , _marker : PhantomData < B > , }
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_IMPL_0011
/* FP:mod.rs-0022 */ impl < 'tcx , B : Bridge > CompilerCtxt < 'tcx , B > { pub fn new (tcx : TyCtxt < 'tcx >) -> Self { Self { tcx , _marker : Default :: default () } } }
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_IMPL_0012
/* FP:mod.rs-0024 */ # [doc = " Implement error handling for extracting function ABI information."] impl < 'tcx , B : Bridge > FnAbiOfHelpers < 'tcx > for CompilerCtxt < 'tcx , B > { type FnAbiOfResult = Result < & 'tcx crate :: rustc_target :: callconv :: FnAbi < 'tcx , Ty < 'tcx > > , B :: Error > ; # [inline] fn handle_fn_abi_err (& self , err : ty :: layout :: FnAbiError < 'tcx > , _span : crate :: rustc_span :: Span , fn_abi_request : ty :: layout :: FnAbiRequest < 'tcx > ,) -> B :: Error { B :: Error :: new (format ! ("Failed to get ABI for `{fn_abi_request:?}`: {err:?}")) } }
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_IMPL_0013
/* FP:mod.rs-0026 */ impl < 'tcx , B : Bridge > LayoutOfHelpers < 'tcx > for CompilerCtxt < 'tcx , B > { type LayoutOfResult = Result < ty :: layout :: TyAndLayout < 'tcx > , B :: Error > ; # [inline] fn handle_layout_err (& self , err : ty :: layout :: LayoutError < 'tcx > , _span : crate :: rustc_span :: Span , ty : Ty < 'tcx > ,) -> B :: Error { B :: Error :: new (format ! ("Failed to get layout for `{ty}`: {err}")) } }
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_IMPL_0014
/* FP:mod.rs-0028 */ impl < 'tcx , B : Bridge > HasTypingEnv < 'tcx > for CompilerCtxt < 'tcx , B > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { ty :: TypingEnv :: fully_monomorphized () } }
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_IMPL_0015
/* FP:mod.rs-0030 */ impl < 'tcx , B : Bridge > HasTyCtxt < 'tcx > for CompilerCtxt < 'tcx , B > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_mod_IMPL_0016
/* FP:mod.rs-0032 */ impl < 'tcx , B : Bridge > HasDataLayout for CompilerCtxt < 'tcx , B > { fn data_layout (& self) -> & crate :: rustc_abi :: TargetDataLayout { self . tcx . data_layout () } }