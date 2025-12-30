// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl fmt :: Display for RustFlagsRepr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { RustFlagsRepr :: U8 => "u8" . fmt (f) , RustFlagsRepr :: U16 => "u16" . fmt (f) , RustFlagsRepr :: U32 => "u32" . fmt (f) , RustFlagsRepr :: U64 => "u64" . fmt (f) , RustFlagsRepr :: U128 => "u128" . fmt (f) , } } }
};
}
