// Generated macro for impl_16 (impl)
macro_rules! Depcrate_ansiimpl_16 {
() => {
// Module: crate::ansi
// Provides: {"impl_16"}
// Dependencies: {}
impl fmt :: Display for Infix { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use crate :: difference :: Difference ; match Difference :: between (& self . 0 , & self . 1) { Difference :: ExtraStyles (style) => { let f : & mut dyn fmt :: Write = f ; style . write_prefix (f) } Difference :: Reset => { let f : & mut dyn fmt :: Write = f ; write ! (f , "{}{}" , RESET , self . 1 . prefix ()) } Difference :: Empty => { Ok (()) } } } }
};
}
