// Generated macro for matches_small_constant_shift (function)
macro_rules! Depcrate_isa_x64_lowermatches_small_constant_shift {
() => {
// Module: crate::isa::x64::lower
// Provides: {"matches_small_constant_shift"}
// Dependencies: {}
# [doc = " Returns whether the given input is a shift by a constant value less or equal than 3."] # [doc = " The goal is to embed it within an address mode."] fn matches_small_constant_shift (ctx : & mut Lower < Inst > , spec : InsnInput) -> Option < (InsnInput , u8) > { matches_input (ctx , spec , Opcode :: Ishl) . and_then (| shift | { match input_to_imm (ctx , InsnInput { insn : shift , input : 1 , } ,) { Some (shift_amt) if shift_amt <= 3 => Some ((InsnInput { insn : shift , input : 0 , } , shift_amt as u8 ,)) , _ => None , } }) }
};
}
