// Generated macro for to_instruction (macro)
macro_rules! Depcrateto_instruction {
() => {
// Module: crate
// Provides: {"to_instruction"}
// Dependencies: {}
macro_rules ! to_instruction { ($ discriminator : expr , $ num : expr , $ num_type : ty) => { { let mut data = [0u8 ; :: core :: mem :: size_of ::<$ num_type > () + 1] ; data [0] = $ discriminator ; data [1 ..] . copy_from_slice (&$ num . to_le_bytes ()) ; Instruction { program_id : id () , data : data . to_vec () , accounts : vec ! [] , } } } ; }
};
}
