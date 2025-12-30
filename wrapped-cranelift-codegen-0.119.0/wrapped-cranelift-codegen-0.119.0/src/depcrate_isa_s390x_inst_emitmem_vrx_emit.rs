// Generated macro for mem_vrx_emit (function)
macro_rules! Depcrate_isa_s390x_inst_emitmem_vrx_emit {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"mem_vrx_emit"}
// Dependencies: {}
pub fn mem_vrx_emit (rd : Reg , mem : & MemArg , opcode : u16 , m3 : u8 , add_trap : bool , sink : & mut MachBuffer < Inst > , emit_info : & EmitInfo , state : & mut EmitState ,) { let (mem_insts , mem) = mem_finalize (mem , state , MemInstType { have_d12 : true , have_d20 : false , have_pcrel : false , have_unaligned_pcrel : false , have_index : true , } ,) ; for inst in mem_insts . into_iter () { inst . emit (sink , emit_info , state) ; } if add_trap { if let Some (trap_code) = mem . get_flags () . trap_code () { sink . add_trap (trap_code) ; } } match & mem { & MemArg :: BXD12 { base , index , disp , .. } => { put (sink , & enc_vrx (opcode , rd , base , index , disp . bits () , m3)) ; } _ => unreachable ! () , } }
};
}
