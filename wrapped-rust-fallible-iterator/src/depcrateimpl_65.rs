// Generated macro for impl_65 (impl)
macro_rules! Depcrateimpl_65 {
() => {
// Module: crate
// Provides: {"impl_65"}
// Dependencies: {}
impl < I : core :: fmt :: Debug , F > core :: fmt :: Debug for Map < I , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("Map") . field ("iter" , & self . it) . finish () } }
};
}
