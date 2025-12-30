// Generated macro for mem_emit (function)
macro_rules! Depcrate_isa_s390x_inst_emitmem_emit {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"mem_emit"}
// Dependencies: {}
pub fn mem_emit (rd : Reg , mem : & MemArg , opcode_rx : Option < u16 > , opcode_rxy : Option < u16 > , opcode_ril : Option < u16 > , add_trap : bool , sink : & mut MachBuffer < Inst > , emit_info : & EmitInfo , state : & mut EmitState ,) { let (mem_insts , mem) = mem_finalize (mem , state , MemInstType { have_d12 : opcode_rx . is_some () , have_d20 : opcode_rxy . is_some () , have_pcrel : opcode_ril . is_some () , have_unaligned_pcrel : opcode_ril . is_some () && ! add_trap , have_index : true , } ,) ; for inst in mem_insts . into_iter () { inst . emit (sink , emit_info , state) ; } if add_trap { if let Some (trap_code) = mem . get_flags () . trap_code () { sink . add_trap (trap_code) ; } } match & mem { & MemArg :: BXD12 { base , index , disp , .. } => { put (sink , & enc_rx (opcode_rx . unwrap () , rd , base , index , disp . bits ()) ,) ; } & MemArg :: BXD20 { base , index , disp , .. } => { put (sink , & enc_rxy (opcode_rxy . unwrap () , rd , base , index , disp . bits ()) ,) ; } & MemArg :: Label { target } => { sink . use_label_at_offset (sink . cur_offset () , target , LabelUse :: BranchRIL) ; put (sink , & enc_ril_b (opcode_ril . unwrap () , rd , 0)) ; } & MemArg :: Symbol { ref name , offset , .. } => { let reloc_offset = sink . cur_offset () + 2 ; sink . add_reloc_at_offset (reloc_offset , Reloc :: S390xPCRel32Dbl , & * * name , (offset + 2) . into () ,) ; put (sink , & enc_ril_b (opcode_ril . unwrap () , rd , 0)) ; } _ => unreachable ! () , } }
};
}
