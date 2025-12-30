// Generated macro for impl_73 (impl)
macro_rules! Depcrate_leavesimpl_73 {
() => {
// Module: crate::leaves
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'a , D : DataType > Property < MTFnMut < D > , D > { # [doc = " Sets the callback for getting a property."] # [doc = ""] # [doc = " For single-thread use."] pub fn on_get < H > (mut self , handler : H) -> Property < MTFnMut < D > , D > where H : 'static + Fn (& mut arg :: IterAppend , & PropInfo < MTFnMut < D > , D >) -> Result < () , MethodErr > { self . get_cb = Some (DebugGetProp (Box :: new (RefCell :: new (handler)) as Box < _ >)) ; self } # [doc = " Sets the callback for setting a property."] # [doc = ""] # [doc = " For single-thread use."] pub fn on_set < H > (mut self , handler : H) -> Property < MTFnMut < D > , D > where H : 'static + Fn (& mut arg :: Iter , & PropInfo < MTFnMut < D > , D >) -> Result < () , MethodErr > { self . set_cb = Some (DebugSetProp (Box :: new (RefCell :: new (handler)) as Box < _ >)) ; self } }
};
}
