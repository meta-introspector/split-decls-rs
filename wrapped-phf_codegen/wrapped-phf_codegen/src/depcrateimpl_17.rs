// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a , T : FmtConst + 'a > fmt :: Display for DisplaySet < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}::Set {{ map: {} }}" , self . inner . path , self . inner) } }
};
}
