// Generated macro for impl_346 (impl)
macro_rules! Depcrate_arg_basic_implimpl_346 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_346"}
// Dependencies: {}
impl RefArg for File { # [inline] fn arg_type (& self) -> ArgType { < File as Arg > :: ARG_TYPE } # [inline] fn signature (& self) -> Signature < 'static > { < File as Arg > :: signature () } # [inline] fn append (& self , i : & mut IterAppend) { < File as Append > :: append_by_ref (self , i) } # [inline] fn as_any (& self) -> & dyn any :: Any { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any { self } # [cfg (unix)] # [inline] fn as_i64 (& self) -> Option < i64 > { Some (self . as_raw_fd () as i64) } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (self . try_clone () . unwrap ()) } }
};
}
