// Generated macro for impl_74 (impl)
macro_rules! Depcrate_leavesimpl_74 {
() => {
// Module: crate::leaves
// Provides: {"impl_74"}
// Dependencies: {}
impl < D : DataType > Property < MTSync < D > , D > { # [doc = " Sets the callback for getting a property."] # [doc = ""] # [doc = " For multi-thread use."] pub fn on_get < H > (mut self , handler : H) -> Property < MTSync < D > , D > where H : Fn (& mut arg :: IterAppend , & PropInfo < MTSync < D > , D >) -> Result < () , MethodErr > + Send + Sync + 'static { self . get_cb = Some (DebugGetProp (Box :: new (handler) as Box < _ >)) ; self } # [doc = " Sets the callback for setting a property."] # [doc = ""] # [doc = " For single-thread use."] pub fn on_set < H > (mut self , handler : H) -> Property < MTSync < D > , D > where H : Fn (& mut arg :: Iter , & PropInfo < MTSync < D > , D >) -> Result < () , MethodErr > + Send + Sync + 'static { self . set_cb = Some (DebugSetProp (Box :: new (handler) as Box < _ >)) ; self } }
};
}
