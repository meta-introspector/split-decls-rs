// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
impl std :: fmt :: Debug for ChallengeRef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("ChallengeRef") . field ("scheme" , & self . scheme) . field ("params" , & ParamsPrinter (& self . params)) . finish () } }
};
}
