// Generated macro for reg_name (function)
macro_rules! Depcrate_isa_pulley_shared_instreg_name {
() => {
// Module: crate::isa::pulley_shared::inst
// Provides: {"reg_name"}
// Dependencies: {}
pub fn reg_name (reg : Reg) -> String { match reg . to_real_reg () { Some (real) => { let n = real . hw_enc () ; match (real . class () , n) { (RegClass :: Int , 63) => format ! ("sp") , (RegClass :: Int , 62) => format ! ("lr") , (RegClass :: Int , 61) => format ! ("fp") , (RegClass :: Int , 60) => format ! ("tmp0") , (RegClass :: Int , 59) => format ! ("tmp1") , (RegClass :: Int , _) => format ! ("x{n}") , (RegClass :: Float , _) => format ! ("f{n}") , (RegClass :: Vector , _) => format ! ("v{n}") , } } None => { format ! ("{reg:?}") } } }
};
}
