// Generated macro for native_arch (function)
macro_rules! Depcrate_cross_compilenative_arch {
() => {
// Module: crate::cross_compile
// Provides: {"native_arch"}
// Dependencies: {}
pub fn native_arch () -> & 'static str { match native () . split ("-") . next () . expect ("Target triple has unexpected format") { "x86_64" => "x86_64" , "aarch64" => "aarch64" , "i686" => "x86" , _ => panic ! ("This test should be gated on cross_compile::disabled.") , } }
};
}
