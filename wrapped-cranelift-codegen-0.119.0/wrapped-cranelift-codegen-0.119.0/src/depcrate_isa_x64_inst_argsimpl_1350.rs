// Generated macro for impl_1350 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1350 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1350"}
// Dependencies: {}
impl fmt :: Debug for ExtMode { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let name = match self { ExtMode :: BL => "bl" , ExtMode :: BQ => "bq" , ExtMode :: WL => "wl" , ExtMode :: WQ => "wq" , ExtMode :: LQ => "lq" , } ; write ! (fmt , "{name}") } }
};
}
