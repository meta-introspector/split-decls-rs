// Generated macro for mem_finalize_for_show (function)
macro_rules! Depcrate_isa_s390x_instmem_finalize_for_show {
() => {
// Module: crate::isa::s390x::inst
// Provides: {"mem_finalize_for_show"}
// Dependencies: {}
fn mem_finalize_for_show (mem : & MemArg , state : & EmitState , mi : MemInstType) -> (String , MemArg) { let (mem_insts , mem) = mem_finalize (mem , state , mi) ; let mut mem_str = mem_insts . into_iter () . map (| inst | inst . print_with_state (& mut EmitState :: default ())) . collect :: < Vec < _ > > () . join (" ; ") ; if ! mem_str . is_empty () { mem_str += " ; " ; } (mem_str , mem) }
};
}
