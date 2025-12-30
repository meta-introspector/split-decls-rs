// Generated macro for impl_72 (impl)
macro_rules! Depcrate_leavesimpl_72 {
() => {
// Module: crate::leaves
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a , D : DataType > Property < MTFn < D > , D > { # [doc = " Sets the callback for getting a property."] # [doc = ""] # [doc = " For single-thread use."] pub fn on_get < H > (mut self , handler : H) -> Property < MTFn < D > , D > where H : 'static + Fn (& mut arg :: IterAppend , & PropInfo < MTFn < D > , D >) -> Result < () , MethodErr > { self . get_cb = Some (DebugGetProp (Box :: new (handler) as Box < _ >)) ; self } # [doc = " Sets the callback for setting a property."] # [doc = ""] # [doc = " For single-thread use."] pub fn on_set < H > (mut self , handler : H) -> Property < MTFn < D > , D > where H : 'static + Fn (& mut arg :: Iter , & PropInfo < MTFn < D > , D >) -> Result < () , MethodErr > { self . set_cb = Some (DebugSetProp (Box :: new (handler) as Box < _ >)) ; self } }
};
}
