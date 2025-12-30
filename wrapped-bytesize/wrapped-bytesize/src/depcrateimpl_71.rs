// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl fmt :: Display for ByteSize { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let display = self . display () ; if f . width () . is_none () { fmt :: Display :: fmt (& display , f) } else { f . pad (& display . to_string ()) } } }
};
}
