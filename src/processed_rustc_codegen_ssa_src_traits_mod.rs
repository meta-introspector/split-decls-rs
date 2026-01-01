/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0001
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0002
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0003
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0004
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0005
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0006
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0007
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0008
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0009
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0010
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0011
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0012
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_MOD_0013
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0014
/* FP:mod.rs-0028 */ use std :: fmt ;
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0015
/* FP:mod.rs-0030 */ use crate :: rustc_complete :: ty :: Ty ;
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0016
/* FP:mod.rs-0032 */ use crate :: rustc_complete :: ty :: layout :: { FnAbiOf , LayoutOf , TyAndLayout } ;
/* FP:mod.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0017
/* FP:mod.rs-0034 */ use crate :: rustc_target :: callconv :: FnAbi ;
/* FP:mod.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0018
/* FP:mod.rs-0036 */ pub use self :: abi :: AbiBuilderMethods ;
/* FP:mod.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0019
/* FP:mod.rs-0038 */ pub use self :: asm :: { AsmBuilderMethods , AsmCodegenMethods , GlobalAsmOperandRef , InlineAsmOperandRef , } ;
/* FP:mod.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0020
/* FP:mod.rs-0040 */ pub use self :: backend :: { BackendTypes , CodegenBackend , ExtraBackendMethods } ;
/* FP:mod.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0021
/* FP:mod.rs-0042 */ pub use self :: builder :: { BuilderMethods , OverflowOp } ;
/* FP:mod.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0022
/* FP:mod.rs-0044 */ pub use self :: consts :: ConstCodegenMethods ;
/* FP:mod.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0023
/* FP:mod.rs-0046 */ pub use self :: coverageinfo :: CoverageInfoBuilderMethods ;
/* FP:mod.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0024
/* FP:mod.rs-0048 */ pub use self :: debuginfo :: { DebugInfoBuilderMethods , DebugInfoCodegenMethods } ;
/* FP:mod.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0025
/* FP:mod.rs-0050 */ pub use self :: declare :: PreDefineCodegenMethods ;
/* FP:mod.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0026
/* FP:mod.rs-0052 */ pub use self :: intrinsic :: IntrinsicCallBuilderMethods ;
/* FP:mod.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0027
/* FP:mod.rs-0054 */ pub use self :: misc :: MiscCodegenMethods ;
/* FP:mod.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0028
/* FP:mod.rs-0056 */ pub use self :: statics :: { StaticBuilderMethods , StaticCodegenMethods } ;
/* FP:mod.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0029
/* FP:mod.rs-0058 */ pub use self :: type_ :: { ArgAbiBuilderMethods , BaseTypeCodegenMethods , DerivedTypeCodegenMethods , LayoutTypeCodegenMethods , TypeCodegenMethods , TypeMembershipCodegenMethods , } ;
/* FP:mod.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_USE_0030
/* FP:mod.rs-0060 */ pub use self :: write :: { ModuleBufferMethods , ThinBufferMethods , WriteBackendMethods } ;
/* FP:mod.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_OTHER_0031
/* FP:mod.rs-0062 */ pub trait CodegenObject = Copy + fmt :: Debug ;
/* FP:mod.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_mod_OTHER_0032
/* FP:mod.rs-0064 */ pub trait CodegenMethods < 'tcx > = LayoutOf < 'tcx , LayoutOfResult = TyAndLayout < 'tcx > > + FnAbiOf < 'tcx , FnAbiOfResult = & 'tcx FnAbi < 'tcx , Ty < 'tcx > > > + TypeCodegenMethods < 'tcx > + ConstCodegenMethods + StaticCodegenMethods + DebugInfoCodegenMethods < 'tcx > + AsmCodegenMethods < 'tcx > + PreDefineCodegenMethods < 'tcx > ;