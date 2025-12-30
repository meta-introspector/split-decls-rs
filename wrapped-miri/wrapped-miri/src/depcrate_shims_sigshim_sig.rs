// Generated macro for shim_sig (macro)
macro_rules! Depcrate_shims_sigshim_sig {
() => {
// Module: crate::shims::sig
// Provides: {"shim_sig"}
// Dependencies: {}
# [doc = " Construct a `ShimSig` with convenient syntax:"] # [doc = " ```rust,ignore"] # [doc = " shim_sig!(this, extern \"C\" fn (*const T, i32) -> usize)"] # [doc = " ```"] # [macro_export] macro_rules ! shim_sig { (extern $ abi : literal fn ($ ($ arg : ty) ,*) -> $ ret : ty) => { | this | $ crate :: shims :: sig :: ShimSig { abi : std :: str :: FromStr :: from_str ($ abi) . expect ("incorrect abi specified") , args : [$ (shim_sig_arg ! (this , $ arg)) ,*] , ret : shim_sig_arg ! (this , $ ret) , } } ; }
};
}
