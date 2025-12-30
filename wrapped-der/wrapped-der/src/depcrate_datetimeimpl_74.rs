// Generated macro for impl_74 (impl)
macro_rules! Depcrate_datetimeimpl_74 {
() => {
// Module: crate::datetime
// Provides: {"impl_74"}
// Dependencies: {}
impl fmt :: Display for DateTime { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:02}-{:02}-{:02}T{:02}:{:02}:{:02}Z" , self . year , self . month , self . day , self . hour , self . minutes , self . seconds) } }
};
}
