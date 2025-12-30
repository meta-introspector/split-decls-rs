// Generated macro for impl_436 (impl)
macro_rules! Depcrate_arg_array_implimpl_436 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_436"}
// Dependencies: {}
impl RefArg for InternalArray { fn arg_type (& self) -> ArgType { ArgType :: Array } fn signature (& self) -> Signature < 'static > { Signature :: from (format ! ("a{}" , self . inner_sig)) } fn append (& self , i : & mut IterAppend) { i . append_container (ArgType :: Array , Some (self . inner_sig . as_cstr ()) , | s | for arg in & self . data { RefArg :: append (arg , s) }) ; } # [inline] fn as_any (& self) -> & dyn any :: Any where Self : 'static { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any where Self : 'static { self } fn as_iter < 'a > (& 'a self) -> Option < Box < dyn Iterator < Item = & 'a dyn RefArg > + 'a > > { Some (Box :: new (self . data . iter () . map (| b | b as & dyn RefArg))) } fn as_static_inner (& self , index : usize) -> Option < & (dyn RefArg + 'static) > where Self : 'static { self . data . get (index) . map (| x | x as & dyn RefArg) } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (InternalArray { data : self . data . iter () . map (| x | x . box_clone ()) . collect () , inner_sig : self . inner_sig . clone () , }) } }
};
}
