// Generated macro for Parameters (struct)
macro_rules! DepcrateParameters {
() => {
// Module: crate
// Provides: {"Parameters"}
// Dependencies: {}
# [doc = " The parameters that were used to construct an [`IoUring`]."] # [doc = ""] # [doc = " This type is a transparent wrapper over the system structure `io_uring_params`. A value can be"] # [doc = " (unsafely) created from any properly laid-out and initialized memory representation."] # [derive (Clone)] # [repr (transparent)] pub struct Parameters (sys :: io_uring_params) ;
};
}
