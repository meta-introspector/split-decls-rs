// Generated macro for impl_174 (impl)
macro_rules! Depcrateimpl_174 {
() => {
// Module: crate
// Provides: {"impl_174"}
// Dependencies: {}
impl core :: fmt :: Debug for DefaultHandler { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("DefaultHandler") . field ("backtrace" , match & self . backtrace { Some (_) => & "Some(Backtrace { ... })" , None => & "None" , } ,) . finish () } }
};
}
