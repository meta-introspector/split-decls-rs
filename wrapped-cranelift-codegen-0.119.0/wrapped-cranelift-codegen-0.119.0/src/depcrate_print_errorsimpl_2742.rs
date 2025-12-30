// Generated macro for impl_2742 (impl)
macro_rules! Depcrate_print_errorsimpl_2742 {
() => {
// Module: crate::print_errors
// Provides: {"impl_2742"}
// Dependencies: {}
impl < 'a > FuncWriter for PrettyVerifierError < 'a > { fn write_block_header (& mut self , w : & mut dyn Write , func : & Function , block : Block , indent : usize ,) -> fmt :: Result { pretty_block_header_error (w , func , block , indent , & mut * self . 0 , self . 1) } fn write_instruction (& mut self , w : & mut dyn Write , func : & Function , aliases : & SecondaryMap < Value , Vec < Value > > , inst : Inst , indent : usize ,) -> fmt :: Result { pretty_instruction_error (w , func , aliases , inst , indent , & mut * self . 0 , self . 1) } fn write_entity_definition (& mut self , w : & mut dyn Write , func : & Function , entity : AnyEntity , value : & dyn fmt :: Display , maybe_fact : Option < & Fact > ,) -> fmt :: Result { pretty_preamble_error (w , func , entity , value , maybe_fact , & mut * self . 0 , self . 1) } }
};
}
