// Generated macro for impl_75 (impl)
macro_rules! Depcrateimpl_75 {
() => {
// Module: crate
// Provides: {"impl_75"}
// Dependencies: {}
impl RustFlagsRepr { fn new (f : & Flags) -> RustFlagsRepr { match f . repr () { FlagsRepr :: U8 => RustFlagsRepr :: U8 , FlagsRepr :: U16 => RustFlagsRepr :: U16 , FlagsRepr :: U32 (1) => RustFlagsRepr :: U32 , FlagsRepr :: U32 (2) => RustFlagsRepr :: U64 , FlagsRepr :: U32 (3 | 4) => RustFlagsRepr :: U128 , FlagsRepr :: U32 (n) => panic ! ("unsupported number of flags: {}" , n * 32) , } } }
};
}
