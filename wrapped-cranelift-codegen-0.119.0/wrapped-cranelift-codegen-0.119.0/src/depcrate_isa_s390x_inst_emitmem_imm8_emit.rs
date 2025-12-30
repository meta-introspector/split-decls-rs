// Generated macro for mem_imm8_emit (function)
macro_rules! Depcrate_isa_s390x_inst_emitmem_imm8_emit {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"mem_imm8_emit"}
// Dependencies: {}
pub fn mem_imm8_emit (imm : u8 , mem : & MemArg , opcode_si : u16 , opcode_siy : u16 , add_trap : bool , sink : & mut MachBuffer < Inst > , emit_info : & EmitInfo , state : & mut EmitState ,) { let (mem_insts , mem) = mem_finalize (mem , state , MemInstType { have_d12 : true , have_d20 : true , have_pcrel : false , have_unaligned_pcrel : false , have_index : false , } ,) ; for inst in mem_insts . into_iter () { inst . emit (sink , emit_info , state) ; } if add_trap { if let Some (trap_code) = mem . get_flags () . trap_code () { sink . add_trap (trap_code) ; } } match & mem { & MemArg :: BXD12 { base , index , disp , .. } => { assert ! (index == zero_reg ()) ; put (sink , & enc_si (opcode_si , base , disp . bits () , imm)) ; } & MemArg :: BXD20 { base , index , disp , .. } => { assert ! (index == zero_reg ()) ; put (sink , & enc_siy (opcode_siy , base , disp . bits () , imm)) ; } _ => unreachable ! () , } }
};
}
