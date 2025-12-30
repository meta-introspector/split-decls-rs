// Generated macro for impl_928 (impl)
macro_rules! Depcrate_ir_layoutimpl_928 {
() => {
// Module: crate::ir::layout
// Provides: {"impl_928"}
// Dependencies: {}
impl < 'f > Iterator for Insts < 'f > { type Item = Inst ; fn next (& mut self) -> Option < Inst > { let rval = self . head ; if let Some (inst) = rval { if self . head == self . tail { self . head = None ; self . tail = None ; } else { self . head = self . layout . insts [inst] . next . into () ; } } rval } }
};
}
