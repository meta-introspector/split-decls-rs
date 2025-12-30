// Generated macro for Array (struct)
macro_rules! Depcrate_arg_array_implArray {
() => {
// Module: crate::arg::array_impl
// Provides: {"Array"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [doc = " Represents a D-Bus Array. Maximum flexibility (wraps an iterator of items to append)."] # [doc = ""] # [doc = " See the argument guide and module level documentation for details and alternatives."] pub struct Array < 'a , T , I > (I , PhantomData < (fn () -> T , & 'a ()) >) ;
};
}
