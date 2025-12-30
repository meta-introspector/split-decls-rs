// Generated macro for reg_name (function)
macro_rules! Depcrate_isa_riscv64_instreg_name {
() => {
// Module: crate::isa::riscv64::inst
// Provides: {"reg_name"}
// Dependencies: {}
pub fn reg_name (reg : Reg) -> String { match reg . to_real_reg () { Some (real) => match real . class () { RegClass :: Int => match real . hw_enc () { 0 => "zero" . into () , 1 => "ra" . into () , 2 => "sp" . into () , 3 => "gp" . into () , 4 => "tp" . into () , 5 ..= 7 => format ! ("t{}" , real . hw_enc () - 5) , 8 => "fp" . into () , 9 => "s1" . into () , 10 ..= 17 => format ! ("a{}" , real . hw_enc () - 10) , 18 ..= 27 => format ! ("s{}" , real . hw_enc () - 16) , 28 ..= 31 => format ! ("t{}" , real . hw_enc () - 25) , _ => unreachable ! () , } , RegClass :: Float => match real . hw_enc () { 0 ..= 7 => format ! ("ft{}" , real . hw_enc () - 0) , 8 ..= 9 => format ! ("fs{}" , real . hw_enc () - 8) , 10 ..= 17 => format ! ("fa{}" , real . hw_enc () - 10) , 18 ..= 27 => format ! ("fs{}" , real . hw_enc () - 16) , 28 ..= 31 => format ! ("ft{}" , real . hw_enc () - 20) , _ => unreachable ! () , } , RegClass :: Vector => format ! ("v{}" , real . hw_enc ()) , } , None => { format ! ("{reg:?}") } } }
};
}
