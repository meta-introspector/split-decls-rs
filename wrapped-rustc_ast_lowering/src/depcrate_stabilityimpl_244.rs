// Generated macro for impl_244 (impl)
macro_rules! Depcrate_stabilityimpl_244 {
() => {
// Module: crate::stability
// Provides: {"impl_244"}
// Dependencies: {}
impl fmt :: Display for UnstableAbi { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self { abi , .. } = self ; match self . explain { GateReason :: Experimental => { write ! (f , "the extern {abi} ABI is experimental and subject to change") } GateReason :: ImplDetail => { write ! (f , "the extern {abi} ABI is an implementation detail and perma-unstable") } } } }
};
}
