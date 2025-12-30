// Generated macro for MethodType (trait)
macro_rules! Depcrate_methodtypeMethodType {
() => {
// Module: crate::methodtype
// Provides: {"MethodType"}
// Dependencies: {}
# [doc = " A helper trait used internally to make the tree generic over MTFn, MTFnMut and MTSync."] # [doc = ""] # [doc = " You should not need to call these methods directly, it's primarily for internal use."] pub trait MethodType < D : DataType > : Sized + Default { # [doc = " For internal use."] type Method : ? Sized ; # [doc = " For internal use."] type GetProp : ? Sized ; # [doc = " For internal use."] type SetProp : ? Sized ; # [doc = " For internal use."] fn call_getprop (_ : & Self :: GetProp , _ : & mut IterAppend , _ : & PropInfo < Self , D >) -> Result < () , MethodErr > ; # [doc = " For internal use."] fn call_setprop (_ : & Self :: SetProp , _ : & mut Iter , _ : & PropInfo < Self , D >) -> Result < () , MethodErr > ; # [doc = " For internal use."] fn call_method (_ : & Self :: Method , _ : & MethodInfo < Self , D >) -> MethodResult ; # [doc = " For internal use."] fn make_getprop < H > (h : H) -> Box < Self :: GetProp > where H : Fn (& mut IterAppend , & PropInfo < Self , D >) -> Result < () , MethodErr > + Send + Sync + 'static ; # [doc = " For internal use."] fn make_method < H > (h : H) -> Box < Self :: Method > where H : Fn (& MethodInfo < Self , D >) -> MethodResult + Send + Sync + 'static ; }
};
}
