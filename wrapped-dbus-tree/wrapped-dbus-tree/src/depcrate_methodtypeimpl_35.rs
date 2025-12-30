// Generated macro for impl_35 (impl)
macro_rules! Depcrate_methodtypeimpl_35 {
() => {
// Module: crate::methodtype
// Provides: {"impl_35"}
// Dependencies: {}
impl < D : DataType > MethodType < D > for MTFnMut < D > { type GetProp = RefCell < dyn FnMut (& mut IterAppend , & PropInfo < Self , D >) -> Result < () , MethodErr > > ; type SetProp = RefCell < dyn FnMut (& mut Iter , & PropInfo < Self , D >) -> Result < () , MethodErr > > ; type Method = RefCell < dyn FnMut (& MethodInfo < Self , D >) -> MethodResult > ; fn call_getprop (p : & Self :: GetProp , i : & mut IterAppend , pinfo : & PropInfo < Self , D >) -> Result < () , MethodErr > { (& mut * p . borrow_mut ()) (i , pinfo) } fn call_setprop (p : & Self :: SetProp , i : & mut Iter , pinfo : & PropInfo < Self , D >) -> Result < () , MethodErr > { (& mut * p . borrow_mut ()) (i , pinfo) } fn call_method (p : & Self :: Method , minfo : & MethodInfo < Self , D >) -> MethodResult { (& mut * p . borrow_mut ()) (minfo) } fn make_getprop < H > (h : H) -> Box < Self :: GetProp > where H : Fn (& mut IterAppend , & PropInfo < Self , D >) -> Result < () , MethodErr > + Send + Sync + 'static { Box :: new (RefCell :: new (h)) } fn make_method < H > (h : H) -> Box < Self :: Method > where H : Fn (& MethodInfo < Self , D >) -> MethodResult + Send + Sync + 'static { Box :: new (RefCell :: new (h)) } }
};
}
