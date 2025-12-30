// Generated macro for mem_finalize_for_show (function)
macro_rules! Depcrate_isa_aarch64_instmem_finalize_for_show {
() => {
// Module: crate::isa::aarch64::inst
// Provides: {"mem_finalize_for_show"}
// Dependencies: {}
fn mem_finalize_for_show (mem : & AMode , access_ty : Type , state : & EmitState) -> (String , String) { let (mem_insts , mem) = mem_finalize (None , mem , access_ty , state) ; let mut mem_str = mem_insts . into_iter () . map (| inst | inst . print_with_state (& mut EmitState :: default ())) . collect :: < Vec < _ > > () . join (" ; ") ; if ! mem_str . is_empty () { mem_str += " ; " ; } let mem = mem . pretty_print (access_ty . bytes () as u8) ; (mem_str , mem) }
};
}
