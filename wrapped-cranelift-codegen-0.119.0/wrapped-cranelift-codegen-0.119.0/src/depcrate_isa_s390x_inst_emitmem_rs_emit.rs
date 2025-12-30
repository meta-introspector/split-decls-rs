// Generated macro for mem_rs_emit (function)
macro_rules! Depcrate_isa_s390x_inst_emitmem_rs_emit {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"mem_rs_emit"}
// Dependencies: {}
pub fn mem_rs_emit (rd : Reg , rn : Reg , mem : & MemArg , opcode_rs : Option < u16 > , opcode_rsy : Option < u16 > , add_trap : bool , sink : & mut MachBuffer < Inst > , emit_info : & EmitInfo , state : & mut EmitState ,) { let (mem_insts , mem) = mem_finalize (mem , state , MemInstType { have_d12 : opcode_rs . is_some () , have_d20 : opcode_rsy . is_some () , have_pcrel : false , have_unaligned_pcrel : false , have_index : false , } ,) ; for inst in mem_insts . into_iter () { inst . emit (sink , emit_info , state) ; } if add_trap { if let Some (trap_code) = mem . get_flags () . trap_code () { sink . add_trap (trap_code) ; } } match & mem { & MemArg :: BXD12 { base , index , disp , .. } => { assert ! (index == zero_reg ()) ; put (sink , & enc_rs (opcode_rs . unwrap () , rd , rn , base , disp . bits ())) ; } & MemArg :: BXD20 { base , index , disp , .. } => { assert ! (index == zero_reg ()) ; put (sink , & enc_rsy (opcode_rsy . unwrap () , rd , rn , base , disp . bits ()) ,) ; } _ => unreachable ! () , } }
};
}
