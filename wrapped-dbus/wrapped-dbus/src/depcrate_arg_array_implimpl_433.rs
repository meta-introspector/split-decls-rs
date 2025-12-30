// Generated macro for impl_433 (impl)
macro_rules! Depcrate_arg_array_implimpl_433 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_433"}
// Dependencies: {}
impl < K : DictKey + RefArg + Clone + 'static > RefArg for InternalDict < K > { fn arg_type (& self) -> ArgType { ArgType :: Array } fn signature (& self) -> Signature < 'static > { self . outer_sig . clone () } fn append (& self , i : & mut IterAppend) { let inner_sig = & self . outer_sig . as_cstr () . to_bytes_with_nul () [1 ..] ; let inner_sig = CStr :: from_bytes_with_nul (inner_sig) . unwrap () ; i . append_container (ArgType :: Array , Some (inner_sig) , | s | for (k , v) in & self . data { s . append_container (ArgType :: DictEntry , None , | ss | { k . append (ss) ; v . append (ss) ; }) }) ; } # [inline] fn as_any (& self) -> & dyn any :: Any where Self : 'static { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any where Self : 'static { self } fn as_iter < 'b > (& 'b self) -> Option < Box < dyn Iterator < Item = & 'b dyn RefArg > + 'b > > { Some (Box :: new (self . data . iter () . flat_map (| (k , v) | vec ! [k as & dyn RefArg , v as & dyn RefArg] . into_iter ()))) } fn as_static_inner (& self , index : usize) -> Option < & (dyn RefArg + 'static) > where Self : 'static { let (k , v) = self . data . get (index / 2) ? ; Some (if index & 1 != 0 { v } else { k }) } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (InternalDict { data : self . data . iter () . map (| (k , v) | (k . clone () , v . box_clone ())) . collect () , outer_sig : self . outer_sig . clone () , }) } }
};
}
