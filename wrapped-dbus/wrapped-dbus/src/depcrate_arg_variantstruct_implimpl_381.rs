// Generated macro for impl_381 (impl)
macro_rules! Depcrate_arg_variantstruct_implimpl_381 {
() => {
// Module: crate::arg::variantstruct_impl
// Provides: {"impl_381"}
// Dependencies: {}
impl RefArg for VecDeque < Box < dyn RefArg > > { fn arg_type (& self) -> ArgType { ArgType :: Struct } fn signature (& self) -> Signature < 'static > { let mut s = String :: from ("(") ; for z in self { s . push_str (& z . signature ()) ; } s . push_str (")") ; Signature :: from (s) } fn append (& self , i : & mut IterAppend) { i . append_container (ArgType :: Struct , None , | s | { for z in self { z . append (s) ; } }) ; } # [inline] fn as_any (& self) -> & dyn any :: Any where Self : 'static { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any where Self : 'static { self } fn as_iter < 'a > (& 'a self) -> Option < Box < dyn Iterator < Item = & 'a dyn RefArg > + 'a > > { Some (Box :: new (self . iter () . map (| b | & * * b))) } # [inline] fn as_static_inner (& self , index : usize) -> Option < & (dyn RefArg + 'static) > where Self : 'static { self . get (index) . map (| x | x as & dyn RefArg) } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { let t : VecDeque < Box < dyn RefArg + 'static > > = self . iter () . map (| x | x . box_clone ()) . collect () ; Box :: new (t) } }
};
}
