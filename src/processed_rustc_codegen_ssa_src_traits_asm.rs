// SRC: ../rust/compiler/rustc_codegen_ssa/src/traits/asm.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{InlineAsmOptions, InlineAsmTemplatePiece};
/* AST_META: AST_ID=2 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=10 | LINES=39 */
use crate::rustc_complete::def_id::DefId;
use crate::rustc_complete::ty::Instance;
use crate::rustc_complete::Span;
use crate::rustc_target::asm::InlineAsmRegOrRegClass;

use super::BackendTypes;
use crate::mir::operand::OperandRef;
use crate::mir::place::PlaceRef;

#[derive(Debug)]
pub enum InlineAsmOperandRef<'tcx, B: BackendTypes + ?Sized> {
    In {
        reg: InlineAsmRegOrRegClass,
        value: OperandRef<'tcx, B::Value>,
    },
    Out {
        reg: InlineAsmRegOrRegClass,
        late: bool,
        place: Option<PlaceRef<'tcx, B::Value>>,
    },
    InOut {
        reg: InlineAsmRegOrRegClass,
        late: bool,
        in_value: OperandRef<'tcx, B::Value>,
        out_place: Option<PlaceRef<'tcx, B::Value>>,
    },
    Const {
        string: String,
    },
    SymFn {
        instance: Instance<'tcx>,
    },
    SymStatic {
        def_id: DefId,
    },
    Label {
        label: B::BasicBlock,
    },
}
/* AST_META: AST_ID=3 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=7 */

#[derive(Debug)]
pub enum GlobalAsmOperandRef<'tcx> {
    Const { string: String },
    SymFn { instance: Instance<'tcx> },
    SymStatic { def_id: DefId },
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=codegen_inline_asm | COMPLEXITY=2 | LINES=14 */

pub trait AsmBuilderMethods<'tcx>: BackendTypes {
    /// Take an inline assembly expression and splat it out via LLVM
    fn codegen_inline_asm(
        &mut self,
        template: &[InlineAsmTemplatePiece],
        operands: &[InlineAsmOperandRef<'tcx, Self>],
        options: InlineAsmOptions,
        line_spans: &[Span],
        instance: Instance<'_>,
        dest: Option<Self::BasicBlock>,
        catch_funclet: Option<(Self::BasicBlock, Option<&Self::Funclet>)>,
    );
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=codegen_global_asm | COMPLEXITY=3 | LINES=17 */

pub trait AsmCodegenMethods<'tcx> {
    fn codegen_global_asm(
        &mut self,
        template: &[InlineAsmTemplatePiece],
        operands: &[GlobalAsmOperandRef<'tcx>],
        options: InlineAsmOptions,
        line_spans: &[Span],
    );

    /// The mangled name of this instance
    ///
    /// Additional mangling is used on
    /// some targets to add a leading underscore (Mach-O)
    /// or byte count suffixes (x86 Windows).
    fn mangled_name(&self, instance: Instance<'tcx>) -> String;
}