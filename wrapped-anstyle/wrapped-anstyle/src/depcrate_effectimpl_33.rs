// Generated macro for impl_33 (impl)
macro_rules! Depcrate_effectimpl_33 {
() => {
// Module: crate::effect
// Provides: {"impl_33"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let effects = (anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE) - anstyle::Effects::BOLD;"] # [doc = " assert_eq!(format!(\"{:?}\", effects), \"Effects(UNDERLINE)\");"] # [doc = " ```"] impl core :: ops :: Sub for Effects { type Output = Self ; # [inline] fn sub (self , other : Self) -> Self { self . remove (other) } }
};
}
