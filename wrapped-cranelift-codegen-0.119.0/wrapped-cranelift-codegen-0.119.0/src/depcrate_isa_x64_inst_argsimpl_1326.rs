// Generated macro for impl_1326 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1326 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1326"}
// Dependencies: {}
impl fmt :: Debug for UnaryRmROpcode { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match self { UnaryRmROpcode :: Bsr => write ! (fmt , "bsr") , UnaryRmROpcode :: Bsf => write ! (fmt , "bsf") , UnaryRmROpcode :: Lzcnt => write ! (fmt , "lzcnt") , UnaryRmROpcode :: Tzcnt => write ! (fmt , "tzcnt") , UnaryRmROpcode :: Popcnt => write ! (fmt , "popcnt") , } } }
};
}
