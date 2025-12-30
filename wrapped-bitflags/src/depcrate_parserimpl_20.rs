// Generated macro for impl_20 (impl)
macro_rules! Depcrate_parserimpl_20 {
() => {
// Module: crate::parser
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'a , B : Flags > fmt :: Display for AsDisplay < 'a , B > where B :: Bits : WriteHex , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { to_writer (self . 0 , f) } }
};
}
