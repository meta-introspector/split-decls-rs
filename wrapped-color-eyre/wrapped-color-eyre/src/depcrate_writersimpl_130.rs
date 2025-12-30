// Generated macro for impl_130 (impl)
macro_rules! Depcrate_writersimpl_130 {
() => {
// Module: crate::writers
// Provides: {"impl_130"}
// Dependencies: {}
# [cfg (feature = "issue-url")] impl < B , H > fmt :: Display for Header < B , H > where B : Display , H : Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f . header (& self . h) . ready () , "{}" , self . body) ? ; Ok (()) } }
};
}
