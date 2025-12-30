// Generated macro for impl_191 (impl)
macro_rules! Depcrate_dfa_accelimpl_191 {
() => {
// Module: crate::dfa::accel
// Provides: {"impl_191"}
// Dependencies: {}
impl core :: fmt :: Debug for Accel { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Accel(") ? ; let mut set = f . debug_set () ; for & b in self . needles () { set . entry (& crate :: util :: escape :: DebugByte (b)) ; } set . finish () ? ; write ! (f , ")") } }
};
}
