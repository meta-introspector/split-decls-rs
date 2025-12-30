// Generated macro for impl_32 (impl)
macro_rules! Depcrate_effectimpl_32 {
() => {
// Module: crate::effect
// Provides: {"impl_32"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut effects = anstyle::Effects::BOLD;"] # [doc = " effects |= anstyle::Effects::UNDERLINE;"] # [doc = " assert_eq!(format!(\"{:?}\", effects), \"Effects(BOLD | UNDERLINE)\");"] # [doc = " ```"] impl core :: ops :: BitOrAssign for Effects { # [inline] fn bitor_assign (& mut self , other : Self) { * self = self . insert (other) ; } }
};
}
