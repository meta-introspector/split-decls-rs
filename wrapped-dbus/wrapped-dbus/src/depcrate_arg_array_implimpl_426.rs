// Generated macro for impl_426 (impl)
macro_rules! Depcrate_arg_array_implimpl_426 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_426"}
// Dependencies: {}
impl < 'a , T , I > RefArg for Array < 'static , T , I > where T : 'a + Arg + RefArg , I : fmt :: Debug + Clone + Send + Sync + Iterator < Item = & 'a T > { fn arg_type (& self) -> ArgType { ArgType :: Array } fn signature (& self) -> Signature < 'static > { Signature :: from (format ! ("a{}" , < T as Arg >:: signature ())) } fn append (& self , i : & mut IterAppend) { let z = self . 0 . clone () ; i . append_container (ArgType :: Array , Some (< T as Arg > :: signature () . as_cstr ()) , | s | for arg in z { RefArg :: append (arg , s) ; }) ; } # [inline] fn as_any (& self) -> & dyn any :: Any where Self : 'static { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any where Self : 'static { self } fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (InternalArray { inner_sig : < T as Arg > :: signature () , data : self . 0 . clone () . map (| x | x . box_clone ()) . collect () , }) } }
};
}
