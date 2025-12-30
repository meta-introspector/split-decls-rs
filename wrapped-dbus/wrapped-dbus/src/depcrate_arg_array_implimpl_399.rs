// Generated macro for impl_399 (impl)
macro_rules! Depcrate_arg_array_implimpl_399 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_399"}
// Dependencies: {}
impl < 'a , T : Arg + RefArg > RefArg for & 'a [T] { fn arg_type (& self) -> ArgType { ArgType :: Array } fn signature (& self) -> Signature < 'static > { Signature :: from (format ! ("a{}" , < T as Arg >:: signature ())) } fn append (& self , i : & mut IterAppend) { array_append (self , i , | arg , s | RefArg :: append (arg , s)) ; } # [inline] fn as_any (& self) -> & dyn any :: Any where Self : 'static { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any where Self : 'static { self } # [inline] fn as_static_inner (& self , index : usize) -> Option < & (dyn RefArg + 'static) > where Self : 'static { self . get (index) . map (| x | x as & dyn RefArg) } fn box_clone (& self) -> Box < dyn RefArg + 'static > { T :: array_clone (self) . unwrap_or_else (| | { Box :: new (InternalArray { inner_sig : < T as Arg > :: signature () , data : self . iter () . map (| x | x . box_clone ()) . collect () , }) }) } }
};
}
