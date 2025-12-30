// Generated macro for impl_680 (impl)
macro_rules! Depcrate_util_alphabetimpl_680 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_680"}
// Dependencies: {}
impl core :: fmt :: Debug for Unit { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { match self . 0 { UnitKind :: U8 (b) => write ! (f , "{:?}" , DebugByte (b)) , UnitKind :: EOI (_) => write ! (f , "EOI") , } } }
};
}
