/* FP:helpers.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_helpers_USE_0001
/* FP:helpers.rs-0002 */ use crate :: rustc_complete :: mir :: interpret :: AllocRange ;
/* FP:helpers.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_helpers_USE_0002
/* FP:helpers.rs-0004 */ use crate :: rustc_complete :: ty ;
/* FP:helpers.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_helpers_USE_0003
/* FP:helpers.rs-0006 */ use crate :: rustc_complete :: ty :: Ty ;
/* FP:helpers.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_helpers_USE_0004
/* FP:helpers.rs-0008 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:helpers.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_helpers_TRAIT_0005
/* FP:helpers.rs-0010 */ pub trait TyHelpers < 'tcx > { fn new_foreign (& self , def_id : DefId) -> Ty < 'tcx > ; }
/* FP:helpers.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_helpers_TRAIT_0006
/* FP:helpers.rs-0012 */ pub trait TypingEnvHelpers < 'tcx > { fn fully_monomorphized (& self) -> ty :: TypingEnv < 'tcx > ; }
/* FP:helpers.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_context_helpers_TRAIT_0007
/* FP:helpers.rs-0014 */ pub trait AllocRangeHelpers < 'tcx > { fn alloc_range (& self , offset : crate :: rustc_abi :: Size , size : crate :: rustc_abi :: Size) -> AllocRange ; }