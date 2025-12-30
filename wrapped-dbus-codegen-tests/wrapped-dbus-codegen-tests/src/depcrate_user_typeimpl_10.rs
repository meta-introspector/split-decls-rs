// Generated macro for impl_10 (impl)
macro_rules! Depcrate_user_typeimpl_10 {
() => {
// Module: crate::user_type
// Provides: {"impl_10"}
// Dependencies: {}
impl RefArg for MyType { fn arg_type (& self) -> ArgType { < MyType as Arg > :: ARG_TYPE } fn signature (& self) -> Signature < 'static > { < MyType as Arg > :: signature () } fn append (& self , i : & mut IterAppend) { < MyType as Append > :: append_by_ref (self , i) } fn as_any (& self) -> & dyn any :: Any { self } fn as_any_mut (& mut self) -> & mut dyn any :: Any { self } fn as_i64 (& self) -> Option < i64 > { None } fn as_u64 (& self) -> Option < u64 > { None } fn as_f64 (& self) -> Option < f64 > { None } fn as_str (& self) -> Option < & str > { None } fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (self . clone ()) } fn array_clone (v : & [Self]) -> Option < Box < dyn RefArg + 'static > > where Self : Sized , { Some (Box :: new (v . to_vec ())) } }
};
}
