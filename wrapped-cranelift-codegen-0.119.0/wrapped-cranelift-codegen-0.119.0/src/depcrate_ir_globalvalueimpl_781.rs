// Generated macro for impl_781 (impl)
macro_rules! Depcrate_ir_globalvalueimpl_781 {
() => {
// Module: crate::ir::globalvalue
// Provides: {"impl_781"}
// Dependencies: {}
impl GlobalValueData { # [doc = " Assume that `self` is an `GlobalValueData::Symbol` and return its name."] pub fn symbol_name (& self) -> & ExternalName { match * self { Self :: Symbol { ref name , .. } => name , _ => panic ! ("only symbols have names") , } } # [doc = " Return the type of this global."] pub fn global_type (& self , isa : & dyn TargetIsa) -> Type { match * self { Self :: VMContext { .. } | Self :: Symbol { .. } => isa . pointer_type () , Self :: IAddImm { global_type , .. } | Self :: Load { global_type , .. } => global_type , Self :: DynScaleTargetConst { .. } => isa . pointer_type () , } } }
};
}
