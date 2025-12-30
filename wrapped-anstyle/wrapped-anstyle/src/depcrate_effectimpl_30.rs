// Generated macro for impl_30 (impl)
macro_rules! Depcrate_effectimpl_30 {
() => {
// Module: crate::effect
// Provides: {"impl_30"}
// Dependencies: {}
# [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let effects = anstyle::Effects::new();"] # [doc = " assert_eq!(format!(\"{:?}\", effects), \"Effects()\");"] # [doc = ""] # [doc = " let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;"] # [doc = " assert_eq!(format!(\"{:?}\", effects), \"Effects(BOLD | UNDERLINE)\");"] # [doc = " ```"] impl core :: fmt :: Debug for Effects { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Effects(") ? ; for (i , index) in self . index_iter () . enumerate () { if i != 0 { write ! (f , " | ") ? ; } write ! (f , "{}" , METADATA [index] . name) ? ; } write ! (f , ")") ? ; Ok (()) } }
};
}
