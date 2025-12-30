// Generated macro for impl_223 (impl)
macro_rules! Depcrate_features_serdeimpl_223 {
() => {
// Module: crate::features::serde
// Provides: {"impl_223"}
// Dependencies: {}
impl < T > core :: fmt :: Debug for Compat < T > where T : core :: fmt :: Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("Compat") . field (& self . 0) . finish () } }
};
}
