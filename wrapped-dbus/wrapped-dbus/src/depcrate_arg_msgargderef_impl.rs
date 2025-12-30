// Generated macro for deref_impl (macro)
macro_rules! Depcrate_arg_msgargderef_impl {
() => {
// Module: crate::arg::msgarg
// Provides: {"deref_impl"}
// Dependencies: {}
macro_rules ! deref_impl { ($ t : ident , $ ss : ident , $ make_mut : expr) => { impl < T : RefArg + ? Sized > RefArg for $ t < T > { # [inline] fn arg_type (& self) -> ArgType { (&** self) . arg_type () } # [inline] fn signature (& self) -> Signature <'static > { (&** self) . signature () } # [inline] fn append (& self , i : & mut IterAppend) { (&** self) . append (i) } # [inline] fn as_any (& self) -> & dyn any :: Any where T : 'static { (&** self) . as_any () } # [inline] fn as_any_mut (& mut $ ss) -> & mut dyn any :: Any where T : 'static { $ make_mut . as_any_mut () } # [inline] fn as_i64 (& self) -> Option < i64 > { (&** self) . as_i64 () } # [inline] fn as_u64 (& self) -> Option < u64 > { (&** self) . as_u64 () } # [inline] fn as_f64 (& self) -> Option < f64 > { (&** self) . as_f64 () } # [inline] fn as_str (& self) -> Option <& str > { (&** self) . as_str () } # [inline] fn as_iter <'a > (&'a self) -> Option < Box < dyn Iterator < Item =&'a dyn RefArg > + 'a >> { (&** self) . as_iter () } # [inline] fn as_static_inner (& self , index : usize) -> Option <& (dyn RefArg + 'static) > where Self : 'static { (&** self) . as_static_inner (index) } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { (&** self) . box_clone () } } impl < T : DictKey > DictKey for $ t < T > { } impl < T : Arg > Arg for $ t < T > { const ARG_TYPE : ArgType = T :: ARG_TYPE ; fn signature () -> Signature <'static > { T :: signature () } } impl <'a , T : Get <'a >> Get <'a > for $ t < T > { fn get (i : & mut Iter <'a >) -> Option < Self > { T :: get (i) . map ($ t :: new) } } } }
};
}
