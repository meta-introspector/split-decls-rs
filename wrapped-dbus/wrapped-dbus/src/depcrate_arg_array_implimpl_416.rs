// Generated macro for impl_416 (impl)
macro_rules! Depcrate_arg_array_implimpl_416 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_416"}
// Dependencies: {}
impl < K : DictKey + RefArg + Eq + Ord , V : RefArg + Arg > RefArg for BTreeMap < K , V > { fn arg_type (& self) -> ArgType { ArgType :: Array } fn signature (& self) -> Signature < 'static > { format ! ("a{{{}{}}}" , < K as Arg >:: signature () , < V as Arg >:: signature ()) . into () } fn append (& self , i : & mut IterAppend) { let sig = CString :: new (format ! ("{{{}{}}}" , < K as Arg >:: signature () , < V as Arg >:: signature ())) . unwrap () ; i . append_container (ArgType :: Array , Some (& sig) , | s | for (k , v) in self { s . append_container (ArgType :: DictEntry , None , | ss | { k . append (ss) ; v . append (ss) ; }) }) ; } # [inline] fn as_any (& self) -> & dyn any :: Any where Self : 'static { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any where Self : 'static { self } fn as_iter < 'b > (& 'b self) -> Option < Box < dyn Iterator < Item = & 'b dyn RefArg > + 'b > > { Some (Box :: new (self . iter () . flat_map (| (k , v) | vec ! [k as & dyn RefArg , v as & dyn RefArg] . into_iter ()))) } fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (InternalDict { outer_sig : self . signature () , data : self . iter () . map (| (k , v) | (k . box_clone () , v . box_clone ())) . collect () , }) } }
};
}
