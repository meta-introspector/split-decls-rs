// Generated macro for impl_71 (impl)
macro_rules! Depcrate_formattersimpl_71 {
() => {
// Module: crate::formatters
// Provides: {"impl_71"}
// Dependencies: {}
impl < T : ToF64 + Unsigned + Copy , O : AsRef < FormatSizeOptions > + Copy > core :: fmt :: Display for SizeFormatter < T , O > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{}" , ISizeFormatter :: from (self)) } }
};
}
