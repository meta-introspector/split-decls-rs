// Generated macro for compatible_scripts (function)
macro_rules! Depcrate_personnames_specifications_derive_localecompatible_scripts {
() => {
// Module: crate::personnames::specifications::derive_locale
// Provides: {"compatible_scripts"}
// Dependencies: {}
fn compatible_scripts (sc1 : subtags :: Script , sc2 : subtags :: Script) -> bool { let jpan_compatible = [script ! ("Hani") , script ! ("Kana") , script ! ("Hira")] ; if sc1 == script ! ("Jpan") && jpan_compatible . contains (& sc2) { return true ; } if sc2 == script ! ("Jpan") && jpan_compatible . contains (& sc1) { return true ; } sc1 == sc2 }
};
}
