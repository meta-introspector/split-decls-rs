// Generated macro for impl_929 (impl)
macro_rules! Depcrate_ir_layoutimpl_929 {
() => {
// Module: crate::ir::layout
// Provides: {"impl_929"}
// Dependencies: {}
impl < 'f > DoubleEndedIterator for Insts < 'f > { fn next_back (& mut self) -> Option < Inst > { let rval = self . tail ; if let Some (inst) = rval { if self . head == self . tail { self . head = None ; self . tail = None ; } else { self . tail = self . layout . insts [inst] . prev . into () ; } } rval } }
};
}
