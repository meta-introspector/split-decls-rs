// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a , T : FmtConst + 'a > fmt :: Display for DisplayOrderedSet < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}::OrderedSet {{ map: {} }}" , self . inner . path , self . inner) } }
};
}
