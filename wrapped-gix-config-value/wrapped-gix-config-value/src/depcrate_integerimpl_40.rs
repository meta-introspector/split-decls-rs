// Generated macro for impl_40 (impl)
macro_rules! Depcrate_integerimpl_40 {
() => {
// Module: crate::integer
// Provides: {"impl_40"}
// Dependencies: {}
impl Display for Integer { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . value) ? ; if let Some (suffix) = self . suffix { write ! (f , "{suffix}") } else { Ok (()) } } }
};
}
