// Generated macro for write_instruction (function)
macro_rules! Depcrate_writewrite_instruction {
() => {
// Module: crate::write
// Provides: {"write_instruction"}
// Dependencies: {}
fn write_instruction (w : & mut dyn Write , func : & Function , aliases : & SecondaryMap < Value , Vec < Value > > , inst : Inst , indent : usize ,) -> fmt :: Result { let mut s = String :: with_capacity (16) ; let srcloc = func . srcloc (inst) ; if ! srcloc . is_default () { write ! (s , "{srcloc} ") ? ; } write ! (w , "{s:indent$}") ? ; let mut has_results = false ; for r in func . dfg . inst_results (inst) { if ! has_results { has_results = true ; write ! (w , "{r}") ? ; } else { write ! (w , ", {r}") ? ; } if let Some (f) = & func . dfg . facts [* r] { write ! (w , " ! {f}") ? ; } } if has_results { write ! (w , " = ") ? ; } let opcode = func . dfg . insts [inst] . opcode () ; match type_suffix (func , inst) { Some (suf) => write ! (w , "{opcode}.{suf}") ? , None => write ! (w , "{opcode}") ? , } write_operands (w , & func . dfg , inst) ? ; writeln ! (w) ? ; for r in func . dfg . inst_results (inst) { write_value_aliases (w , aliases , * r , indent) ? ; } Ok (()) }
};
}
