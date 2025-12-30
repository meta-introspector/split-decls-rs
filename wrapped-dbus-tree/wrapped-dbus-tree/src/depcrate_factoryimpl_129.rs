// Generated macro for impl_129 (impl)
macro_rules! Depcrate_factoryimpl_129 {
() => {
// Module: crate::factory
// Provides: {"impl_129"}
// Dependencies: {}
impl < D : DataType > Factory < MTFnMut < D > , D > { # [doc = " Creates a new method for single-thread use."] pub fn method < H , T > (& self , t : T , data : D :: Method , handler : H) -> Method < MTFnMut < D > , D > where H : 'static + FnMut (& MethodInfo < MTFnMut < D > , D >) -> MethodResult , T : Into < Member < 'static > > { super :: leaves :: new_method (t . into () , data , Box :: new (RefCell :: new (handler)) as Box < _ >) } }
};
}
