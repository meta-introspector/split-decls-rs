// SRC: ../rust/compiler/rustc_codegen_ssa/src/traits/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=5 | LINES=31 */
// Interface of a Rust codegen backend
//
// This crate defines all the traits that have to be implemented by a codegen backend in order to
// use the backend-agnostic codegen code in `rustc_codegen_ssa`.
//
// The interface is designed around two backend-specific data structures, the codegen context and
// the builder. The codegen context is supposed to be read-only after its creation and during the
// actual codegen, while the builder stores the information about the function during codegen and
// is used to produce the instructions of the backend IR.
//
// The traits contain associated types that are backend-specific, such as the backend's value or
// basic blocks.


use std::fmt;

use crate::rustc_complete::ty::Ty;
use crate::rustc_complete::ty::layout::{FnAbiOf, LayoutOf, TyAndLayout};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */
use crate::rustc_target::callconv::FnAbi;

pub use self::abi::AbiBuilderMethods;
pub use self::asm::{
    AsmBuilderMethods, AsmCodegenMethods, GlobalAsmOperandRef, InlineAsmOperandRef,
};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
pub use self::backend::{BackendTypes, CodegenBackend, ExtraBackendMethods};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
pub use self::builder::{BuilderMethods, OverflowOp};
/* AST_META: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
pub use self::consts::ConstCodegenMethods;
pub use self::coverageinfo::CoverageInfoBuilderMethods;
pub use self::debuginfo::{DebugInfoBuilderMethods, DebugInfoCodegenMethods};
/* AST_META: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
pub use self::declare::PreDefineCodegenMethods;
pub use self::intrinsic::IntrinsicCallBuilderMethods;
pub use self::misc::MiscCodegenMethods;
pub use self::statics::{StaticBuilderMethods, StaticCodegenMethods};
/* AST_META: AST_ID=7 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
pub use self::type_::{
    ArgAbiBuilderMethods, BaseTypeCodegenMethods, DerivedTypeCodegenMethods,
    LayoutTypeCodegenMethods, TypeCodegenMethods, TypeMembershipCodegenMethods,
};
/* AST_META: AST_ID=8 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
pub use self::write::{ModuleBufferMethods, ThinBufferMethods, WriteBackendMethods};
/* AST_META: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=1 | LINES=11 */

pub trait CodegenObject = Copy + fmt::Debug;

pub trait CodegenMethods<'tcx> = LayoutOf<'tcx, LayoutOfResult = TyAndLayout<'tcx>>
    + FnAbiOf<'tcx, FnAbiOfResult = &'tcx FnAbi<'tcx, Ty<'tcx>>>
    + TypeCodegenMethods<'tcx>
    + ConstCodegenMethods
    + StaticCodegenMethods
    + DebugInfoCodegenMethods<'tcx>
    + AsmCodegenMethods<'tcx>
    + PreDefineCodegenMethods<'tcx>;