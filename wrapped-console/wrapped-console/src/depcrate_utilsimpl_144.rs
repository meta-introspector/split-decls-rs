// Generated macro for impl_144 (impl)
macro_rules! Depcrate_utilsimpl_144 {
() => {
// Module: crate::utils
// Provides: {"impl_144"}
// Dependencies: {}
impl fmt :: Display for Emoji < '_ , '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if wants_emoji () { write ! (f , "{}" , self . 0) } else { write ! (f , "{}" , self . 1) } } }
};
}
