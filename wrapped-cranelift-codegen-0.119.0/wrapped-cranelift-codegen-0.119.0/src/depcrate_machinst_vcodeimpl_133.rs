// Generated macro for impl_133 (impl)
macro_rules! Depcrate_machinst_vcodeimpl_133 {
() => {
// Module: crate::machinst::vcode
// Provides: {"impl_133"}
// Dependencies: {}
impl < I : VCodeInst > fmt :: Debug for VCode < I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { writeln ! (f , "VCode {{") ? ; writeln ! (f , "  Entry block: {}" , self . entry . index ()) ? ; let mut state = Default :: default () ; for block in 0 .. self . num_blocks () { let block = BlockIndex :: new (block) ; writeln ! (f , "Block {}({:?}):" , block . index () , self . block_params (block)) ? ; if let Some (bb) = self . bindex_to_bb (block) { writeln ! (f , "    (original IR block: {bb})") ? ; } for (succ_idx , succ) in self . block_succs (block) . iter () . enumerate () { writeln ! (f , "    (successor: Block {}({:?}))" , succ . index () , self . branch_blockparams (block , InsnIndex :: new (0) , succ_idx)) ? ; } for inst in self . block_ranges . get (block . index ()) { writeln ! (f , "  Inst {}: {}" , inst , self . insts [inst] . pretty_print_inst (& mut state)) ? ; if ! self . operands . is_empty () { for operand in self . inst_operands (InsnIndex :: new (inst)) { if operand . kind () == OperandKind :: Def { if let Some (fact) = & self . facts [operand . vreg () . vreg ()] { writeln ! (f , "    v{} ! {}" , operand . vreg () . vreg () , fact) ? ; } } } } if let Some (user_stack_map) = self . get_user_stack_map (InsnIndex :: new (inst)) { writeln ! (f , "    {user_stack_map:?}") ? ; } } } writeln ! (f , "}}") ? ; Ok (()) } }
};
}
