// Generated macro for impl_188 (impl)
macro_rules! Depcrate_dfa_accelimpl_188 {
() => {
// Module: crate::dfa::accel
// Provides: {"impl_188"}
// Dependencies: {}
impl core :: fmt :: Debug for Accel { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Accel(") ? ; let mut set = f . debug_set () ; for & b in self . needles () { set . entry (& crate :: util :: escape :: DebugByte (b)) ; } set . finish () ? ; write ! (f , ")") } }
};
}
