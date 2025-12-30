// Generated macro for integer_impl (macro)
macro_rules! Depcrate_arg_basic_implinteger_impl {
() => {
// Module: crate::arg::basic_impl
// Provides: {"integer_impl"}
// Dependencies: {}
macro_rules ! integer_impl { ($ t : ident , $ s : ident , $ f : expr , $ i : ident , $ ii : expr , $ u : ident , $ uu : expr , $ fff : ident , $ ff : expr) => { impl Arg for $ t { const ARG_TYPE : ArgType = ArgType ::$ s ; # [inline] fn signature () -> Signature <'static > { unsafe { Signature :: from_slice_unchecked ($ f) } } } impl Append for $ t { fn append_by_ref (& self , i : & mut IterAppend) { arg_append_basic (& mut i . 0 , ArgType ::$ s , * self) } } impl <'a > Get <'a > for $ t { fn get (i : & mut Iter) -> Option < Self > { arg_get_basic (& mut i . 0 , ArgType ::$ s) } } impl RefArg for $ t { # [inline] fn arg_type (& self) -> ArgType { ArgType ::$ s } # [inline] fn signature (& self) -> Signature <'static > { unsafe { Signature :: from_slice_unchecked ($ f) } } # [inline] fn append (& self , i : & mut IterAppend) { arg_append_basic (& mut i . 0 , ArgType ::$ s , * self) } # [inline] fn as_any (& self) -> & dyn any :: Any { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any { self } # [inline] fn as_i64 (& self) -> Option < i64 > { let $ i = * self ; $ ii } # [inline] fn as_u64 (& self) -> Option < u64 > { let $ u = * self ; $ uu } # [inline] fn as_f64 (& self) -> Option < f64 > { let $ fff = * self ; $ ff } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (self . clone ()) } fn array_clone (v : & [Self]) -> Option < Box < dyn RefArg + 'static >> where Self : Sized { Some (Box :: new (v . to_vec ())) } } impl DictKey for $ t { } unsafe impl FixedArray for $ t { } } }
};
}
