// Generated macro for retract (function)
macro_rules! Depcrate_instructionretract {
() => {
// Module: crate::instruction
// Provides: {"retract"}
// Dependencies: {}
# [doc = " Returns the instructions required to retract a program."] # [cfg (feature = "bincode")] pub fn retract (program_address : & Pubkey , authority : & Pubkey) -> Instruction { Instruction :: new_with_bincode (id () , & LoaderV4Instruction :: Retract , vec ! [AccountMeta :: new (* program_address , false) , AccountMeta :: new_readonly (* authority , true) ,] ,) }
};
}
