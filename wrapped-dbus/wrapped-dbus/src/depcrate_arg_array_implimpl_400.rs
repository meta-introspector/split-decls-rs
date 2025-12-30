// Generated macro for impl_400 (impl)
macro_rules! Depcrate_arg_array_implimpl_400 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_400"}
// Dependencies: {}
impl < T : Arg + RefArg > RefArg for Vec < T > { fn arg_type (& self) -> ArgType { ArgType :: Array } fn signature (& self) -> Signature < 'static > { Signature :: from (format ! ("a{}" , < T as Arg >:: signature ())) } fn append (& self , i : & mut IterAppend) { array_append (& self , i , | arg , s | RefArg :: append (arg , s)) ; } # [inline] fn as_any (& self) -> & dyn any :: Any where Self : 'static { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any where Self : 'static { self } fn as_iter < 'a > (& 'a self) -> Option < Box < dyn Iterator < Item = & 'a dyn RefArg > + 'a > > { Some (Box :: new (self . iter () . map (| b | b as & dyn RefArg))) } # [inline] fn as_static_inner (& self , index : usize) -> Option < & (dyn RefArg + 'static) > where Self : 'static { self . get (index) . map (| x | x as & dyn RefArg) } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { (& * * self) . box_clone () } }
};
}
