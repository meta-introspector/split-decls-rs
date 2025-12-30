// Generated macro for impl_248 (impl)
macro_rules! Depcrate_arg_msgargimpl_248 {
() => {
// Module: crate::arg::msgarg
// Provides: {"impl_248"}
// Dependencies: {}
impl < 'a , T : RefArg + ? Sized > RefArg for & 'a T { # [inline] fn arg_type (& self) -> ArgType { (& * * self) . arg_type () } # [inline] fn signature (& self) -> Signature < 'static > { (& * * self) . signature () } # [inline] fn append (& self , i : & mut IterAppend) { (& * * self) . append (i) } # [inline] fn as_any (& self) -> & dyn any :: Any where T : 'static { (& * * self) . as_any () } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any where T : 'static { unreachable ! () } # [inline] fn as_i64 (& self) -> Option < i64 > { (& * * self) . as_i64 () } # [inline] fn as_u64 (& self) -> Option < u64 > { (& * * self) . as_u64 () } # [inline] fn as_f64 (& self) -> Option < f64 > { (& * * self) . as_f64 () } # [inline] fn as_str (& self) -> Option < & str > { (& * * self) . as_str () } # [inline] fn as_iter < 'b > (& 'b self) -> Option < Box < dyn Iterator < Item = & 'b dyn RefArg > + 'b > > { (& * * self) . as_iter () } # [inline] fn as_static_inner (& self , index : usize) -> Option < & (dyn RefArg + 'static) > where Self : 'static { (& * * self) . as_static_inner (index) } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { (& * * self) . box_clone () } }
};
}
