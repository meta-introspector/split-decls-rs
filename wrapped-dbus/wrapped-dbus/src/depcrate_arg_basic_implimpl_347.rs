// Generated macro for impl_347 (impl)
macro_rules! Depcrate_arg_basic_implimpl_347 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_347"}
// Dependencies: {}
# [cfg (all (unix , feature = "io-lifetimes"))] impl RefArg for io_lifetimes :: OwnedFd { # [inline] fn arg_type (& self) -> ArgType { < Self as Arg > :: ARG_TYPE } # [inline] fn signature (& self) -> Signature < 'static > { < Self as Arg > :: signature () } # [inline] fn append (& self , i : & mut IterAppend) { < Self as Append > :: append_by_ref (self , i) } # [inline] fn as_any (& self) -> & dyn any :: Any { self } # [inline] fn as_any_mut (& mut self) -> & mut dyn any :: Any { self } # [cfg (unix)] # [inline] fn as_i64 (& self) -> Option < i64 > { Some (self . as_raw_fd () as i64) } # [inline] fn box_clone (& self) -> Box < dyn RefArg + 'static > { Box :: new (self . try_clone () . unwrap ()) } }
};
}
