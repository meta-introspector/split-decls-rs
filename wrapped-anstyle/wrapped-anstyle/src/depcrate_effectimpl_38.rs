// Generated macro for impl_38 (impl)
macro_rules! Depcrate_effectimpl_38 {
() => {
// Module: crate::effect
// Provides: {"impl_38"}
// Dependencies: {}
impl core :: fmt :: Display for EffectsDisplay { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { for index in self . 0 . index_iter () { f . write_str (METADATA [index] . escape) ? ; } Ok (()) } }
};
}
