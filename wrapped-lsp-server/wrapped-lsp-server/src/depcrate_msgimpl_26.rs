// Generated macro for impl_26 (impl)
macro_rules! Depcrate_msgimpl_26 {
() => {
// Module: crate::msg
// Provides: {"impl_26"}
// Dependencies: {}
impl fmt :: Display for RequestId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . 0 { IdRepr :: I32 (it) => fmt :: Display :: fmt (it , f) , IdRepr :: String (it) => fmt :: Debug :: fmt (it , f) , } } }
};
}
