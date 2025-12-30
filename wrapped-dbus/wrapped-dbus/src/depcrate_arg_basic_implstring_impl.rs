// Generated macro for string_impl (macro)
macro_rules! Depcrate_arg_basic_implstring_impl {
() => {
// Module: crate::arg::basic_impl
// Provides: {"string_impl"}
// Dependencies: {}
macro_rules ! string_impl { ($ t : ident , $ s : ident , $ f : expr) => { impl <'a > Arg for $ t <'a > { const ARG_TYPE : ArgType = ArgType ::$ s ; fn signature () -> Signature <'static > { unsafe { Signature :: from_slice_unchecked ($ f) } } } impl RefArg for $ t <'static > { fn arg_type (& self) -> ArgType { ArgType ::$ s } fn signature (& self) -> Signature <'static > { unsafe { Signature :: from_slice_unchecked ($ f) } } fn append (& self , i : & mut IterAppend) { arg_append_str (& mut i . 0 , ArgType ::$ s , self . as_cstr ()) } # [inline] fn as_any (& self) -> & dyn any :: Any { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any { self } # [inline] fn as_str (& self) -> Option <& str > { Some (self) } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (self . clone () . into_static ()) } fn array_clone (v : & [Self]) -> Option < Box < dyn RefArg + 'static >> where Self : Sized { Some (Box :: new (v . to_vec ())) } } impl <'a > DictKey for $ t <'a > { } impl <'a > Append for $ t <'a > { fn append_by_ref (& self , i : & mut IterAppend) { arg_append_str (& mut i . 0 , ArgType ::$ s , self . as_cstr ()) } } impl <'a > Get <'a > for $ t <'static > { fn get (i : & mut Iter <'a >) -> Option <$ t <'static >> { unsafe { let c = arg_get_str (& mut i . 0 , ArgType ::$ s) ?; let s = std :: str :: from_utf8 (c . to_bytes_with_nul ()) . ok () ?; Some ($ t :: from_slice_unchecked (s) . into_static ()) } } } } }
};
}
