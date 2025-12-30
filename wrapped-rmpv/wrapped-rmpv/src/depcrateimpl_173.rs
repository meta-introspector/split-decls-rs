// Generated macro for impl_173 (impl)
macro_rules! Depcrateimpl_173 {
() => {
// Module: crate
// Provides: {"impl_173"}
// Dependencies: {}
impl Display for Integer { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match self . n { IntPriv :: PosInt (v) => Display :: fmt (& v , fmt) , IntPriv :: NegInt (v) => Display :: fmt (& v , fmt) , } } }
};
}
