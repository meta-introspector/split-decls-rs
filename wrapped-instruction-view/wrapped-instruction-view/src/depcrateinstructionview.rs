// Generated macro for InstructionView (struct)
macro_rules! DepcrateInstructionView {
() => {
// Module: crate
// Provides: {"InstructionView"}
// Dependencies: {}
# [doc = " Information about an instruction."] # [derive (Debug , Clone)] pub struct InstructionView < 'a , 'b , 'c , 'd > where 'a : 'b , { # [doc = " Address of the program."] pub program_id : & 'c Address , # [doc = " Data expected by the program instruction."] pub data : & 'd [u8] , # [doc = " Metadata describing the accounts that should be passed to the program."] pub accounts : & 'b [InstructionAccount < 'a >] , }
};
}
