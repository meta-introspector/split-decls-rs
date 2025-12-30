// Generated macro for impl_987 (impl)
macro_rules! Depcrateimpl_987 {
() => {
// Module: crate
// Provides: {"impl_987"}
// Dependencies: {}
impl fmt :: Display for Dir { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use Dir :: * ; f . pad (match * self { Bi => "bidirectional" , Uni => "unidirectional" , }) } }
};
}
