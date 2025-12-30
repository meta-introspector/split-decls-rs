// Generated macro for type_suffix (function)
macro_rules! Depcrate_writetype_suffix {
() => {
// Module: crate::write
// Provides: {"type_suffix"}
// Dependencies: {}
fn type_suffix (func : & Function , inst : Inst) -> Option < Type > { let inst_data = & func . dfg . insts [inst] ; let constraints = inst_data . opcode () . constraints () ; if ! constraints . is_polymorphic () { return None ; } if constraints . use_typevar_operand () { let ctrl_var = inst_data . typevar_operand (& func . dfg . value_lists) . unwrap () ; let def_block = match func . dfg . value_def (ctrl_var) { ValueDef :: Result (instr , _) => func . layout . inst_block (instr) , ValueDef :: Param (block , _) => Some (block) , ValueDef :: Union (..) => None , } ; if def_block . is_some () && def_block == func . layout . inst_block (inst) { return None ; } } let rtype = func . dfg . ctrl_typevar (inst) ; assert ! (! rtype . is_invalid () , "Polymorphic instruction must produce a result") ; Some (rtype) }
};
}
