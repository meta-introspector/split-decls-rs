// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < T : Debug , G : TearDown > Debug for Fixture < T , G > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> Result < () , std :: fmt :: Error > { write ! (f , "Fixture<{:?}>" , self . inner) } }
};
}
