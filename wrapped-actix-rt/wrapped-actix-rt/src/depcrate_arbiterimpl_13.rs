// Generated macro for impl_13 (impl)
macro_rules! Depcrate_arbiterimpl_13 {
() => {
// Module: crate::arbiter
// Provides: {"impl_13"}
// Dependencies: {}
impl fmt :: Debug for ArbiterCommand { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ArbiterCommand :: Stop => write ! (f , "ArbiterCommand::Stop") , ArbiterCommand :: Execute (_) => write ! (f , "ArbiterCommand::Execute") , } } }
};
}
