// Generated macro for impl_18 (impl)
macro_rules! Depcrate_ansiimpl_18 {
() => {
// Module: crate::ansi
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Display for Infix { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match Difference :: between (& self . 0 , & self . 1) { Difference :: ExtraStyles (style) => { let f : & mut dyn fmt :: Write = f ; style . write_prefix (f) } , Difference :: Reset => { let f : & mut dyn fmt :: Write = f ; write ! (f , "{}{}" , RESET , self . 1 . prefix ()) } , Difference :: NoDifference => { Ok (()) } , } } }
};
}
