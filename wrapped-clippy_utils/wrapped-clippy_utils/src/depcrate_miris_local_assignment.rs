// Generated macro for is_local_assignment (function)
macro_rules! Depcrate_miris_local_assignment {
() => {
// Module: crate::mir
// Provides: {"is_local_assignment"}
// Dependencies: {}
fn is_local_assignment (mir : & Body < '_ > , local : Local , location : Location) -> bool { let Location { block , statement_index } = location ; let basic_block = & mir . basic_blocks [block] ; if statement_index < basic_block . statements . len () { let statement = & basic_block . statements [statement_index] ; if let StatementKind :: Assign (box (place , _)) = statement . kind { place . as_local () == Some (local) } else { false } } else { let terminator = basic_block . terminator () ; match & terminator . kind { TerminatorKind :: Call { destination , .. } => destination . as_local () == Some (local) , TerminatorKind :: InlineAsm { operands , .. } => operands . iter () . any (| operand | { if let InlineAsmOperand :: Out { place : Some (place) , .. } = operand { place . as_local () == Some (local) } else { false } }) , _ => false , } } }
};
}
