// Generated macro for impl_33 (impl)
macro_rules! Depcrate_methodtypeimpl_33 {
() => {
// Module: crate::methodtype
// Provides: {"impl_33"}
// Dependencies: {}
impl < D : DataType > MethodType < D > for MTFn < D > { type GetProp = dyn Fn (& mut IterAppend , & PropInfo < Self , D >) -> Result < () , MethodErr > ; type SetProp = dyn Fn (& mut Iter , & PropInfo < Self , D >) -> Result < () , MethodErr > ; type Method = dyn Fn (& MethodInfo < Self , D >) -> MethodResult ; fn call_getprop (p : & Self :: GetProp , i : & mut IterAppend , pinfo : & PropInfo < Self , D >) -> Result < () , MethodErr > { p (i , pinfo) } fn call_setprop (p : & Self :: SetProp , i : & mut Iter , pinfo : & PropInfo < Self , D >) -> Result < () , MethodErr > { p (i , pinfo) } fn call_method (p : & Self :: Method , minfo : & MethodInfo < Self , D >) -> MethodResult { p (minfo) } fn make_getprop < H > (h : H) -> Box < Self :: GetProp > where H : Fn (& mut IterAppend , & PropInfo < Self , D >) -> Result < () , MethodErr > + Send + Sync + 'static { Box :: new (h) } fn make_method < H > (h : H) -> Box < Self :: Method > where H : Fn (& MethodInfo < Self , D >) -> MethodResult + Send + Sync + 'static { Box :: new (h) } }
};
}
