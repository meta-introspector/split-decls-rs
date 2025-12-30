// Generated macro for impl_228 (impl)
macro_rules! Depcrate_features_serdeimpl_228 {
() => {
// Module: crate::features::serde
// Provides: {"impl_228"}
// Dependencies: {}
impl < T > core :: fmt :: Debug for BorrowCompat < T > where T : core :: fmt :: Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("BorrowCompat") . field (& self . 0) . finish () } }
};
}
