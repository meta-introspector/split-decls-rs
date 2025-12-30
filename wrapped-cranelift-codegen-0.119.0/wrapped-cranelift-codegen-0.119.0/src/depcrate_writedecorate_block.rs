// Generated macro for decorate_block (function)
macro_rules! Depcrate_writedecorate_block {
() => {
// Module: crate::write
// Provides: {"decorate_block"}
// Dependencies: {}
fn decorate_block < FW : FuncWriter > (func_w : & mut FW , w : & mut dyn Write , func : & Function , aliases : & SecondaryMap < Value , Vec < Value > > , block : Block ,) -> fmt :: Result { let indent = if func . rel_srclocs () . is_empty () { 4 } else { 36 } ; func_w . write_block_header (w , func , block , indent) ? ; for a in func . dfg . block_params (block) . iter () . cloned () { write_value_aliases (w , aliases , a , indent) ? ; } for inst in func . layout . block_insts (block) { func_w . write_instruction (w , func , aliases , inst , indent) ? ; } Ok (()) }
};
}
