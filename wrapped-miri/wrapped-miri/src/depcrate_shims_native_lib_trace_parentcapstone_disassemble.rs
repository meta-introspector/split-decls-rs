// Generated macro for capstone_disassemble (function)
macro_rules! Depcrate_shims_native_lib_trace_parentcapstone_disassemble {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"capstone_disassemble"}
// Dependencies: {}
# [doc = " Extract the events from the given instruction."] fn capstone_disassemble (instr : & [u8] , addr : usize , cs : & capstone :: Capstone , acc_events : & mut Vec < AccessEvent > ,) -> capstone :: CsResult < () > { let insns = cs . disasm_count (instr , 0x1000 , 1) ? ; let ins_detail = cs . insn_detail (& insns [0]) ? ; let arch_detail = ins_detail . arch_detail () ; let mut found_mem_op = false ; for op in arch_detail . operands () { if capstone_find_events (addr , & op , acc_events) { if found_mem_op { panic ! ("more than one memory operand found; we don't know which one accessed what") ; } found_mem_op = true ; } } Ok (()) }
};
}
