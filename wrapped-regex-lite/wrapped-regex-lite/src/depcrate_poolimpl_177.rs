// Generated macro for impl_177 (impl)
macro_rules! Depcrate_poolimpl_177 {
() => {
// Module: crate::pool
// Provides: {"impl_177"}
// Dependencies: {}
impl < T : core :: fmt :: Debug , F > core :: fmt :: Debug for Pool < T , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("Pool") . field ("stack" , & self . stack) . finish () } }
};
}
