// Generated macro for impl_2858 (impl)
macro_rules! Depcrate_writeimpl_2858 {
() => {
// Module: crate::write
// Provides: {"impl_2858"}
// Dependencies: {}
impl FuncWriter for PlainWriter { fn write_instruction (& mut self , w : & mut dyn Write , func : & Function , aliases : & SecondaryMap < Value , Vec < Value > > , inst : Inst , indent : usize ,) -> fmt :: Result { write_instruction (w , func , aliases , inst , indent) } fn write_block_header (& mut self , w : & mut dyn Write , func : & Function , block : Block , indent : usize ,) -> fmt :: Result { write_block_header (w , func , block , indent) } }
};
}
