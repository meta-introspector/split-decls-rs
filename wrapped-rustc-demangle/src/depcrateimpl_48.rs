// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a > fmt :: Display for DemangleStyle < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { DemangleStyle :: Legacy (ref d) => fmt :: Display :: fmt (d , f) , DemangleStyle :: V0 (ref d) => fmt :: Display :: fmt (d , f) , } } }
};
}
