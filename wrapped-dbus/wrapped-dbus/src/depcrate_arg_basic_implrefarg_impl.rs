// Generated macro for refarg_impl (macro)
macro_rules! Depcrate_arg_basic_implrefarg_impl {
() => {
// Module: crate::arg::basic_impl
// Provides: {"refarg_impl"}
// Dependencies: {}
macro_rules ! refarg_impl { ($ t : ty , $ i : ident , $ ii : expr , $ ss : expr , $ uu : expr , $ ff : expr) => { impl RefArg for $ t { # [inline] fn arg_type (& self) -> ArgType { <$ t as Arg >:: ARG_TYPE } # [inline] fn signature (& self) -> Signature <'static > { <$ t as Arg >:: signature () } # [inline] fn append (& self , i : & mut IterAppend) { <$ t as Append >:: append_by_ref (self , i) } # [inline] fn as_any (& self) -> & dyn any :: Any { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any { self } # [inline] fn as_i64 (& self) -> Option < i64 > { let $ i = self ; $ ii } # [inline] fn as_u64 (& self) -> Option < u64 > { let $ i = self ; $ uu } # [inline] fn as_f64 (& self) -> Option < f64 > { let $ i = self ; $ ff } # [inline] fn as_str (& self) -> Option <& str > { let $ i = self ; $ ss } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (self . clone ()) } fn array_clone (v : & [Self]) -> Option < Box < dyn RefArg + 'static >> where Self : Sized { Some (Box :: new (v . to_vec ())) } } } }
};
}
