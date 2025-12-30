// Generated macro for impl_130 (impl)
macro_rules! Depcrate_factoryimpl_130 {
() => {
// Module: crate::factory
// Provides: {"impl_130"}
// Dependencies: {}
impl < D : DataType > Factory < MTSync < D > , D > { # [doc = " Creates a new method for multi-thread use."] pub fn method < H , T > (& self , t : T , data : D :: Method , handler : H) -> Method < MTSync < D > , D > where H : Fn (& MethodInfo < MTSync < D > , D >) -> MethodResult + Send + Sync + 'static , T : Into < Member < 'static > > { super :: leaves :: new_method (t . into () , data , Box :: new (handler) as Box < _ >) } }
};
}
