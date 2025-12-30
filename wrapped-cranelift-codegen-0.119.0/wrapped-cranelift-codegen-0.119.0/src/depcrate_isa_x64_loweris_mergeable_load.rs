// Generated macro for is_mergeable_load (function)
macro_rules! Depcrate_isa_x64_loweris_mergeable_load {
() => {
// Module: crate::isa::x64::lower
// Provides: {"is_mergeable_load"}
// Dependencies: {}
# [doc = " Determines whether a load operation (indicated by `src_insn`) can be merged"] # [doc = " into the current lowering point. If so, returns the address-base source (as"] # [doc = " an `InsnInput`) and an offset from that address from which to perform the"] # [doc = " load."] fn is_mergeable_load (ctx : & mut Lower < Inst > , src_insn : IRInst , size : MergeableLoadSize ,) -> Option < (InsnInput , i32) > { let insn_data = ctx . data (src_insn) ; let inputs = ctx . num_inputs (src_insn) ; if inputs != 1 { return None ; } let load_ty = ctx . output_ty (src_insn , 0) ; if ty_bits (load_ty) < 32 { match size { MergeableLoadSize :: Exact => { } MergeableLoadSize :: Min32 => return None , } } if let & InstructionData :: Load { opcode : Opcode :: Load , offset , .. } = insn_data { Some ((InsnInput { insn : src_insn , input : 0 , } , offset . into () ,)) } else { None } }
};
}
