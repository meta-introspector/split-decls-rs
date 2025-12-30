// Generated macro for InlineAssemblyGenerator (struct)
macro_rules! Depcrate_inline_asmInlineAssemblyGenerator {
() => {
// Module: crate::inline_asm
// Provides: {"InlineAssemblyGenerator"}
// Dependencies: {}
struct InlineAssemblyGenerator < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , arch : InlineAsmArch , enclosing_def_id : DefId , template : & 'a [InlineAsmTemplatePiece] , operands : & 'a [CInlineAsmOperand < 'tcx >] , options : InlineAsmOptions , registers : Vec < Option < InlineAsmReg > > , stack_slots_clobber : Vec < Option < Size > > , stack_slots_input : Vec < Option < Size > > , stack_slots_output : Vec < Option < Size > > , stack_slot_size : Size , }
};
}
