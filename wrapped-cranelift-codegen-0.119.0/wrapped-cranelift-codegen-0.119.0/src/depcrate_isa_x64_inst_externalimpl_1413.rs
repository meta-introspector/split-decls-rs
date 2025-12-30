// Generated macro for impl_1413 (impl)
macro_rules! Depcrate_isa_x64_inst_externalimpl_1413 {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"impl_1413"}
// Dependencies: {}
impl < 'a , T : OperandVisitor > asm :: RegisterVisitor < CraneliftRegisters > for RegallocVisitor < 'a , T > { fn read (& mut self , reg : & mut Gpr) { self . collector . reg_use (reg) ; } fn read_write (& mut self , reg : & mut PairedGpr) { let PairedGpr { read , write } = reg ; self . collector . reg_use (read) ; self . collector . reg_reuse_def (write , 0) ; } fn fixed_read (& mut self , _reg : & Gpr) { todo ! () } fn fixed_read_write (& mut self , _reg : & PairedGpr) { todo ! () } fn read_xmm (& mut self , reg : & mut Xmm) { self . collector . reg_use (reg) ; } fn read_write_xmm (& mut self , reg : & mut PairedXmm) { let PairedXmm { read , write } = reg ; self . collector . reg_use (read) ; self . collector . reg_reuse_def (write , 0) ; } fn fixed_read_xmm (& mut self , _reg : & Xmm) { todo ! () } fn fixed_read_write_xmm (& mut self , _reg : & PairedXmm) { todo ! () } }
};
}
