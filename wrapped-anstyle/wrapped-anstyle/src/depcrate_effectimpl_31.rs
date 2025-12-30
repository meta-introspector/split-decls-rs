// Generated macro for impl_31 (impl)
macro_rules! Depcrate_effectimpl_31 {
() => {
// Module: crate::effect
// Provides: {"impl_31"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;"] # [doc = " assert_eq!(format!(\"{:?}\", effects), \"Effects(BOLD | UNDERLINE)\");"] # [doc = " ```"] impl core :: ops :: BitOr for Effects { type Output = Self ; # [inline (always)] fn bitor (self , rhs : Self) -> Self { self . insert (rhs) } }
};
}
