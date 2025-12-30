// Generated macro for impl_677 (impl)
macro_rules! Depcrate_util_alphabetimpl_677 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_677"}
// Dependencies: {}
impl core :: fmt :: Debug for Unit { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { match self . 0 { UnitKind :: U8 (b) => write ! (f , "{:?}" , DebugByte (b)) , UnitKind :: EOI (_) => write ! (f , "EOI") , } } }
};
}
