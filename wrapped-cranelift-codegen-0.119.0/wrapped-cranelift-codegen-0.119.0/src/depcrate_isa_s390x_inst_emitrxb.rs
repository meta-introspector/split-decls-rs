// Generated macro for rxb (function)
macro_rules! Depcrate_isa_s390x_inst_emitrxb {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"rxb"}
// Dependencies: {}
fn rxb (v1 : Option < Reg > , v2 : Option < Reg > , v3 : Option < Reg > , v4 : Option < Reg >) -> u8 { let mut rxb = 0 ; let is_high_vr = | reg | -> bool { if let Some (reg) = reg { if ! is_fpr (reg) { return true ; } } false } ; if is_high_vr (v1) { rxb = rxb | 8 ; } if is_high_vr (v2) { rxb = rxb | 4 ; } if is_high_vr (v3) { rxb = rxb | 2 ; } if is_high_vr (v4) { rxb = rxb | 1 ; } rxb }
};
}
