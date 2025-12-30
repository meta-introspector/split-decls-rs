// Generated macro for impl_128 (impl)
macro_rules! Depcrate_factoryimpl_128 {
() => {
// Module: crate::factory
// Provides: {"impl_128"}
// Dependencies: {}
impl < D : DataType > Factory < MTFn < D > , D > { # [doc = " Creates a new method for single-thread use."] pub fn method < H , T > (& self , t : T , data : D :: Method , handler : H) -> Method < MTFn < D > , D > where H : 'static + Fn (& MethodInfo < MTFn < D > , D >) -> MethodResult , T : Into < Member < 'static > > { super :: leaves :: new_method (t . into () , data , Box :: new (handler) as Box < _ >) } }
};
}
