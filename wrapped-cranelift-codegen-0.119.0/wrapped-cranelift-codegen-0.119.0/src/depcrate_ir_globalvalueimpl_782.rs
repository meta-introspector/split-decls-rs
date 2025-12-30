// Generated macro for impl_782 (impl)
macro_rules! Depcrate_ir_globalvalueimpl_782 {
() => {
// Module: crate::ir::globalvalue
// Provides: {"impl_782"}
// Dependencies: {}
impl fmt :: Display for GlobalValueData { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Self :: VMContext => write ! (f , "vmctx") , Self :: Load { base , offset , global_type , flags , } => write ! (f , "load.{global_type}{flags} {base}{offset}") , Self :: IAddImm { global_type , base , offset , } => write ! (f , "iadd_imm.{global_type} {base}, {offset}") , Self :: Symbol { ref name , offset , colocated , tls , } => { write ! (f , "symbol {}{}{}" , if colocated { "colocated " } else { "" } , if tls { "tls " } else { "" } , name . display (None)) ? ; let offset_val : i64 = offset . into () ; if offset_val > 0 { write ! (f , "+") ? ; } if offset_val != 0 { write ! (f , "{offset}") ? ; } Ok (()) } Self :: DynScaleTargetConst { vector_type } => { write ! (f , "dyn_scale_target_const.{vector_type}") } } } }
};
}
