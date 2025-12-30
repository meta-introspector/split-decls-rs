// Generated macro for impl_367 (impl)
macro_rules! Depcrate_arg_variantstruct_implimpl_367 {
() => {
// Module: crate::arg::variantstruct_impl
// Provides: {"impl_367"}
// Dependencies: {}
impl < T : RefArg > RefArg for Variant < T > { fn arg_type (& self) -> ArgType { ArgType :: Variant } fn signature (& self) -> Signature < 'static > { unsafe { Signature :: from_slice_unchecked ("v\0") } } fn append (& self , i : & mut IterAppend) { let z = & self . 0 ; i . append_container (ArgType :: Variant , Some (z . signature () . as_cstr ()) , | s | z . append (s)) ; } # [inline] fn as_any (& self) -> & dyn any :: Any where T : 'static { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any where T : 'static { self } # [inline] fn as_i64 (& self) -> Option < i64 > { self . 0 . as_i64 () } # [inline] fn as_u64 (& self) -> Option < u64 > { self . 0 . as_u64 () } # [inline] fn as_f64 (& self) -> Option < f64 > { self . 0 . as_f64 () } # [inline] fn as_str (& self) -> Option < & str > { self . 0 . as_str () } # [inline] fn as_iter < 'a > (& 'a self) -> Option < Box < dyn Iterator < Item = & 'a dyn RefArg > + 'a > > { use std :: iter ; let z : & dyn RefArg = & self . 0 ; Some (Box :: new (iter :: once (z))) } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (Variant (self . 0 . box_clone ())) } # [inline] fn as_static_inner (& self , index : usize) -> Option < & (dyn RefArg + 'static) > where Self : 'static { if index == 0 { Some (& self . 0) } else { None } } }
};
}
