// Generated macro for impl_711 (impl)
macro_rules! Depcrate_ir_extfuncimpl_711 {
() => {
// Module: crate::ir::extfunc
// Provides: {"impl_711"}
// Dependencies: {}
impl fmt :: Display for ArgumentPurpose { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match self { Self :: Normal => "normal" , Self :: StructArgument (size) => return write ! (f , "sarg({size})") , Self :: StructReturn => "sret" , Self :: VMContext => "vmctx" , }) } }
};
}
