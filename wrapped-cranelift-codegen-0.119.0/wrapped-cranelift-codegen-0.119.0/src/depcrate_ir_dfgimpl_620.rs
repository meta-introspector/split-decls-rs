// Generated macro for impl_620 (impl)
macro_rules! Depcrate_ir_dfgimpl_620 {
() => {
// Module: crate::ir::dfg
// Provides: {"impl_620"}
// Dependencies: {}
impl < 'a > fmt :: Display for DisplayInst < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let dfg = self . 0 ; let inst = self . 1 ; if let Some ((first , rest)) = dfg . inst_results (inst) . split_first () { write ! (f , "{first}") ? ; for v in rest { write ! (f , ", {v}") ? ; } write ! (f , " = ") ? ; } let typevar = dfg . ctrl_typevar (inst) ; if typevar . is_invalid () { write ! (f , "{}" , dfg . insts [inst] . opcode ()) ? ; } else { write ! (f , "{}.{}" , dfg . insts [inst] . opcode () , typevar) ? ; } write_operands (f , dfg , inst) } }
};
}
