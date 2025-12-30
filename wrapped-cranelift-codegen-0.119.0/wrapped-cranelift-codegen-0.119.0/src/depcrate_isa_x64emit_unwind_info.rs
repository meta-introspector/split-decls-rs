// Generated macro for emit_unwind_info (function)
macro_rules! Depcrate_isa_x64emit_unwind_info {
() => {
// Module: crate::isa::x64
// Provides: {"emit_unwind_info"}
// Dependencies: {}
# [doc = " Emit unwind info for an x86 target."] pub fn emit_unwind_info (buffer : & MachBufferFinalized < Final > , kind : crate :: isa :: unwind :: UnwindInfoKind ,) -> CodegenResult < Option < crate :: isa :: unwind :: UnwindInfo > > { use crate :: isa :: unwind :: { UnwindInfo , UnwindInfoKind } ; Ok (match kind { UnwindInfoKind :: SystemV => { let mapper = self :: inst :: unwind :: systemv :: RegisterMapper ; Some (UnwindInfo :: SystemV (crate :: isa :: unwind :: systemv :: create_unwind_info_from_insts (& buffer . unwind_info [..] , buffer . data () . len () , & mapper ,) ? ,)) } UnwindInfoKind :: Windows => Some (UnwindInfo :: WindowsX64 (crate :: isa :: unwind :: winx64 :: create_unwind_info_from_insts :: < self :: inst :: unwind :: winx64 :: RegisterMapper , > (& buffer . unwind_info [..]) ? ,)) , _ => None , }) }
};
}
