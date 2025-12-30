// Generated macro for impl_355 (impl)
macro_rules! Depcrateimpl_355 {
() => {
// Module: crate
// Provides: {"impl_355"}
// Dependencies: {}
impl core :: fmt :: Display for TryGetError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { write ! (f , "Not enough bytes remaining in buffer to read value (requested {} but only {} available)" , self . requested , self . available) } }
};
}
