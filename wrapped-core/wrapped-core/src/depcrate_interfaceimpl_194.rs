// Generated macro for impl_194 (impl)
macro_rules! Depcrate_interfaceimpl_194 {
() => {
// Module: crate::interface
// Provides: {"impl_194"}
// Dependencies: {}
impl < I : core :: fmt :: Debug + Interface > core :: fmt :: Debug for InterfaceRef < '_ , I > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { < I as core :: fmt :: Debug > :: fmt (& * * self , f) } }
};
}
